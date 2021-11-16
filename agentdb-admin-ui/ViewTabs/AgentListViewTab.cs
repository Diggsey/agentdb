using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace AgentdbAdmin
{
    public partial class AgentListViewTab : UserControl, IViewTab
    {
        private ConnectionTab parent;
        private AgentdbAdmin.IOpaqueHandle connectionHandle;
        private List<byte> root;
        private Guid from = Guid.Empty;
        private uint limit = 100;
        private bool reverse = false;
        private bool skipFirst = false;
        private List<Guid> agentIds = new List<Guid>();

        public AgentListViewTab(ConnectionTab parent, AgentdbAdmin.IOpaqueHandle connectionHandle, List<byte> root)
        {
            this.parent = parent;
            this.connectionHandle = connectionHandle;
            this.root = root;
            this.Dock = DockStyle.Fill;
            InitializeComponent();
            PerformRefresh();
        }

        public async void PerformRefresh()
        {
            agentIds = (await parent.MainForm.PerformAsync<List<Guid>>("Listing agents", continuation =>
            {
                AgentdbAdmin.ListAgents(connectionHandle, root, from, limit + (skipFirst ? 1u : 0u), reverse, continuation);
            }));
            if (skipFirst)
            {
                agentIds.RemoveAt(0);
            }
            if (reverse)
            {
                agentIds.Reverse();
            }
            SuspendLayout();
            agentListBox.BeginUpdate();
            var selectedItem = agentListBox.SelectedItem;
            agentListBox.Items.Clear();
            foreach (var agentId in agentIds)
            {
                agentListBox.Items.Add(agentId);
            }
            agentListBox.SelectedItem = selectedItem;
            agentListBox.EndUpdate();
            ResumeLayout();
        }

        private void agentsPerPageBox_ValueChanged(object sender, EventArgs e)
        {
            limit = (uint)agentsPerPageBox.Value;
        }

        private void nextPageButton_Click(object sender, EventArgs e)
        {
            from = agentIds.LastOrDefault();
            skipFirst = agentIds.Count > 0;
            reverse = false;
            PerformRefresh();
        }

        private void prevPageButton_Click(object sender, EventArgs e)
        {
            from = (agentIds.Count > 0) ? agentIds.First() : new Guid(
                uint.MaxValue, ushort.MaxValue, ushort.MaxValue,
                byte.MaxValue, byte.MaxValue, byte.MaxValue, byte.MaxValue,
                byte.MaxValue, byte.MaxValue, byte.MaxValue, byte.MaxValue
            );
            skipFirst = agentIds.Count > 0;
            reverse = true;
            PerformRefresh();
        }

        private void agentListBox_MouseDoubleClick(object sender, MouseEventArgs e)
        {
            var index = agentListBox.IndexFromPoint(e.Location);
            if (e.Button == MouseButtons.Left && index != ListBox.NoMatches)
            {
                var agentId = ((Guid)agentListBox.Items[index]);
                parent.OpenPage(new ConnectionTab.BlobPageId() { Root = root, BlobId = agentId });
            }
        }

        private void agentListBox_KeyDown(object sender, KeyEventArgs e)
        {
            if (e.KeyCode == Keys.Enter && agentListBox.SelectedItem != null)
            {
                parent.OpenPage(new ConnectionTab.BlobPageId() { Root = root, BlobId = (Guid)agentListBox.SelectedItem });
            }
        }
    }
}
