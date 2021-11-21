using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using static System.Windows.Forms.ListViewItem;

namespace AgentdbAdmin
{
    public partial class RootViewTab : UserControl, IViewTab
    {
        private ConnectionTab parent;
        AgentdbAdmin.IOpaqueHandle connectionHandle;
        private string root;
        private AgentdbAdmin.RootDesc rootDesc;
        private TreeNode overviewNode;
        private TreeNode clientsNode;
        private TreeNode partitionsNode;

        struct PartitionGroup
        {
            public uint Index { get; set; }
        }

        public RootViewTab(ConnectionTab parent, AgentdbAdmin.IOpaqueHandle connectionHandle, string root)
        {
            this.parent = parent;
            this.connectionHandle = connectionHandle;
            this.root = root;
            this.Dock = DockStyle.Fill;
            InitializeComponent();

            overviewNode = treeView.Nodes["OverviewNode"];
            clientsNode = treeView.Nodes["ClientsNode"];
            partitionsNode = treeView.Nodes["PartitionsNode"];

            PerformRefresh();
        }

        private void UpdateSelection()
        {
            var tag = treeView.SelectedNode?.Tag;
            var client = tag as AgentdbAdmin.ClientDesc?;
            var partition = tag as KeyValuePair<uint, AgentdbAdmin.PartitionDesc>?;
            var partitionGroup = tag as PartitionGroup?;

            detailsTableLayout.SuspendLayout();

            lastActiveBox.Visible = client.HasValue;
            lastActiveLabel.Visible = client.HasValue;
            sendPartitionBox.Visible = tag == null;
            sendPartitionLabel.Visible = tag == null;
            recvPartitionBox.Visible = tag == null;
            recvPartitionLabel.Visible = tag == null;
            repartitionButton.Visible = tag == null;
            actionsLabel.Visible = repartitionButton.Visible;

            var selectedPartitions = new SortedSet<uint>();

            if (client.HasValue)
            {
                detailsNameBox.Text = client.Value.name;
                detailsTypeBox.Text = "Client";
                lastActiveBox.Text = client.Value.lastActiveTs.ToString(Utils.DateFormat);
                includedPartitionsBox.Text = $"{client.Value.partitions.Item1} to {client.Value.partitions.Item2 - 1}";
                for (var p = client.Value.partitions.Item1; p < client.Value.partitions.Item2; ++p)
                {
                    selectedPartitions.Add(p);
                }
            } else if (partition.HasValue)
            {
                detailsNameBox.Text = "Partition " + partition.Value.Key;
                detailsTypeBox.Text = "Partition";
                selectedPartitions.Add(partition.Value.Key);
                includedPartitionsBox.Text = partition.Value.Key.ToString();
            } else if (partitionGroup.HasValue)
            {
                detailsNameBox.Text = $"Partitions {partitionGroup.Value.Index*10} to {partitionGroup.Value.Index*10+9}";
                detailsTypeBox.Text = "Partition Group";
                for (uint p = 0; p < 10; ++p)
                {
                    selectedPartitions.Add(partitionGroup.Value.Index * 10 + p);
                }
                includedPartitionsBox.Text = $"{partitionGroup.Value.Index*10} to {partitionGroup.Value.Index*10+9}";
            }
            else if (rootDesc.partitions != null)
            {
                detailsNameBox.Text = root;
                detailsTypeBox.Text = "Root";
                sendPartitionBox.Text = $"{rootDesc.partitionRangeSend.Item1} to {rootDesc.partitionRangeSend.Item2 - 1}";
                recvPartitionBox.Text = $"{rootDesc.partitionRangeRecv.Item1} to {rootDesc.partitionRangeRecv.Item2 - 1}";
                selectedPartitions.UnionWith(rootDesc.partitions.Keys);

                if (rootDesc.partitionRangeRecv == rootDesc.partitionRangeSend)
                {
                    includedPartitionsBox.Text = recvPartitionBox.Text;
                } else
                {
                    includedPartitionsBox.Text = $"{recvPartitionBox.Text}, {sendPartitionBox.Text}";
                }
            }

            long agentCount = 0;
            messagesView.BeginUpdate();
            messagesView.Items.Clear();
            bool didOverflow = false;
            foreach (var partitionIndex in selectedPartitions)
            {
                AgentdbAdmin.PartitionDesc partitionDesc;
                if (rootDesc.partitions.TryGetValue(partitionIndex, out partitionDesc))
                {
                    var partitionIndexStr = partitionIndex.ToString();
                    agentCount += partitionDesc.agentCount;

                    foreach (var message in partitionDesc.batchedMessages)
                    {
                        var item = new ListViewItem(new string[] {
                            partitionIndexStr,
                            "Yes",
                            message.scheduledFor.HasValue ? message.scheduledFor.Value.ToString(Utils.DateFormat) : "Now",
                            message.messageId.ToString(),
                            message.recipientId.ToString(),
                        });
                        item.Tag = message;
                        messagesView.Items.Add(item);
                    }

                    foreach (var message in partitionDesc.pendingMessages)
                    {
                        var item = new ListViewItem(new string[] {
                            partitionIndexStr,
                            "No",
                            message.scheduledFor.HasValue ? message.scheduledFor.Value.ToString(Utils.DateFormat) : "Now",
                            message.messageId.ToString(),
                            message.recipientId.ToString(),
                        });
                        item.Tag = message;
                        messagesView.Items.Add(item);
                    }
                    didOverflow |= partitionDesc.batchedMessagesOverflow || partitionDesc.pendingMessagesOverflow;
                }
            }

            if (didOverflow)
            {
                var item = new ListViewItem(new string[] {
                    "-",
                    "-",
                    "-",
                    "<Some messages omitted>",
                    "",
                });
                messagesView.Items.Add(item);
            }
            messagesView.EndUpdate();

            agentCountBox.Text = agentCount.ToString();

            detailsTableLayout.ResumeLayout();
        }

        public async void PerformRefresh()
        {
            rootDesc = await parent.MainForm.PerformAsync<AgentdbAdmin.RootDesc>("Loading root", continuation =>
            {
                AgentdbAdmin.DescribeRoot(connectionHandle, this.root, continuation);
            });

            treeView.BeginUpdate();
            var selectedName = treeView.SelectedNode?.Name;
            TreeNode nodeToSelect = null;
            clientsNode.Nodes.Clear();
            foreach (var clientDesc in rootDesc.clients)
            {
                var clientNode = new TreeNode(clientDesc.name);
                clientNode.Name = $"Client {clientDesc.name}";
                clientNode.Tag = clientDesc;
                clientsNode.Nodes.Add(clientNode);

                if (clientNode.Name == selectedName)
                {
                    nodeToSelect = clientNode;
                }
            }

            partitionsNode.Nodes.Clear();
            uint prevGroupIndex = uint.MaxValue;
            TreeNode groupNode = null;
            foreach (var kvp in rootDesc.partitions)
            {
                var groupIndex = kvp.Key / 10;
                if (groupIndex != prevGroupIndex)
                {
                    prevGroupIndex = groupIndex;
                    groupNode = new TreeNode($"Partitions {groupIndex * 10} to {groupIndex * 10 + 9}");
                    groupNode.Name = groupNode.Text;
                    groupNode.Tag = new PartitionGroup() { Index = groupIndex };
                    partitionsNode.Nodes.Add(groupNode);

                    if (groupNode.Name == selectedName)
                    {
                        nodeToSelect = groupNode;
                    }
                }

                var partitionNode = new TreeNode("Partition " + kvp.Key);
                partitionNode.Name = partitionNode.Text;
                partitionNode.Tag = kvp;
                groupNode.Nodes.Add(partitionNode);

                if (partitionNode.Name == selectedName)
                {
                    nodeToSelect = partitionNode;
                }
            }
            if (nodeToSelect != null)
            {
                treeView.SelectedNode = nodeToSelect;
            } else
            {
                UpdateSelection();
            }
            treeView.EndUpdate();
        }

        private void treeView_AfterSelect(object sender, TreeViewEventArgs e)
        {
            UpdateSelection();
        }

        private void copyDetailsToolStripMenuItem_Click(object sender, EventArgs e)
        {
            var content = string.Join(Environment.NewLine, Enumerable.Cast<ListViewItem>(messagesView.SelectedItems).Select(item => {
                return Utils.FormatCsv(Enumerable.Cast<ListViewSubItem>(item.SubItems).Select(subItem => subItem.Text));
            }));

            Clipboard.SetText(content);
        }

        private void copyMessageIDToolStripMenuItem_Click(object sender, EventArgs e)
        {
            var content = string.Join("\n", Enumerable.Cast<ListViewItem>(
                messagesView.SelectedItems).Select(item => item.SubItems[messagesIdColumn.Index].Text
            ));

            Clipboard.SetText(content);
        }

        private void copyRecipientIDToolStripMenuItem_Click(object sender, EventArgs e)
        {
            var content = string.Join("\n", Enumerable.Cast<ListViewItem>(
                messagesView.SelectedItems).Select(item => item.SubItems[messagesRecipientIdColumn.Index].Text
            ));

            Clipboard.SetText(content);
        }

        private void OpenMessages()
        {
            foreach (var item in Enumerable.Cast<ListViewItem>(messagesView.SelectedItems))
            {
                var message = item.Tag as AgentdbAdmin.MessageDesc?;
                if (message.HasValue)
                {
                    parent.OpenPage(new ConnectionTab.BlobPageId() {
                        Root = root,
                        BlobId = message.Value.messageId,
                    });
                }
            }
        }

        private void messagesView_MouseDoubleClick(object sender, MouseEventArgs e)
        {
            OpenMessages();
        }

        private void viewMessageToolStripMenuItem_Click(object sender, EventArgs e)
        {
            OpenMessages();
        }

        private void viewRecipientToolStripMenuItem_Click(object sender, EventArgs e)
        {
            foreach (var item in Enumerable.Cast<ListViewItem>(messagesView.SelectedItems))
            {
                var message = item.Tag as AgentdbAdmin.MessageDesc?;
                if (message.HasValue)
                {
                    parent.OpenPage(new ConnectionTab.BlobPageId()
                    {
                        Root = root,
                        BlobId = message.Value.recipientId,
                    });
                }
            }
        }

        private async void repartitionButton_Click(object sender, EventArgs e)
        {
            var dialog = new Modals.RepartitionModal();
            dialog.RootName = root;
            dialog.PartitionRecvRange = rootDesc.partitionRangeRecv;
            dialog.PartitionSendRange = rootDesc.partitionRangeSend;
            dialog.NewPartitionRange = rootDesc.partitionRangeSend;
            if (dialog.ShowDialog(parent.MainForm) == DialogResult.OK)
            {
                await parent.MainForm.PerformAsync<AgentdbAdmin.NoResult>("Re-partitioning root", continuation =>
                {
                    AgentdbAdmin.ChangePartitions(connectionHandle, this.root, dialog.NewPartitionRange, continuation);
                });
            }
        }

        private void listAgentsButton_Click(object sender, EventArgs e)
        {
            parent.OpenPage(new ConnectionTab.ListAgentsPageId() { Root = root });
        }
    }
}
