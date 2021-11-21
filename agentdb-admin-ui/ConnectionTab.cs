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
    public partial class ConnectionTab : UserControl
    {
        class RootItem
        {
            public string Root { get; set; }
            public string Title => Root;
        }

        public abstract class PageId
        {
            public abstract string Title { get;  }
            public abstract IViewTab CreateTab(ConnectionTab parent, AgentdbAdmin.IOpaqueHandle connectionHandle);
        }

        public class RootPageId: PageId
        {
            public string Root { get; set; }
            public override string Title => Root;

            public override IViewTab CreateTab(ConnectionTab parent, AgentdbAdmin.IOpaqueHandle connectionHandle)
            {
                return new RootViewTab(parent, connectionHandle, Root);
            }

            public override bool Equals(object obj)
            {
                return obj is RootPageId id &&
                       EqualityComparer<string>.Default.Equals(Root, id.Root);
            }

            public override int GetHashCode()
            {
                return -1490287827 + EqualityComparer<string>.Default.GetHashCode(Root);
            }
        }

        public class ListAgentsPageId : PageId
        {
            public string Root { get; set; }
            public override string Title => Root + ": Agent list";

            public override IViewTab CreateTab(ConnectionTab parent, AgentdbAdmin.IOpaqueHandle connectionHandle)
            {
                return new AgentListViewTab(parent, connectionHandle, Root);
            }

            public override bool Equals(object obj)
            {
                return obj is ListAgentsPageId id && Root == id.Root;
            }

            public override int GetHashCode()
            {
                return -1490287827 + EqualityComparer<string>.Default.GetHashCode(Root);
            }
        }

        public class BlobPageId : PageId
        {
            public string Root { get; set; }
            public Guid BlobId { get; set; }

            public override string Title => Root + ": " + BlobId.ToString();

            public override IViewTab CreateTab(ConnectionTab parent, AgentdbAdmin.IOpaqueHandle connectionHandle)
            {
                return new BlobViewTab(parent, connectionHandle, Root, BlobId);
            }

            public override bool Equals(object obj)
            {
                return obj is BlobPageId id &&
                       EqualityComparer<string>.Default.Equals(Root, id.Root) &&
                       BlobId.Equals(id.BlobId);
            }

            public override int GetHashCode()
            {
                int hashCode = 1616903828;
                hashCode = hashCode * -1521134295 + EqualityComparer<string>.Default.GetHashCode(Root);
                hashCode = hashCode * -1521134295 + BlobId.GetHashCode();
                return hashCode;
            }
        }

        public MainForm MainForm { get; set; }
        private AgentdbAdmin.IOpaqueHandle connectionHandle;
        private Dictionary<PageId, TabPage> tabPages = new Dictionary<PageId, TabPage>();
        private List<TabPage> tabHistory = new List<TabPage>();
        private bool closingTab = false;

        public ConnectionTab(MainForm parent, AgentdbAdmin.IOpaqueHandle connectionHandle)
        {
            this.MainForm = parent;
            this.connectionHandle = connectionHandle;
            this.Dock = DockStyle.Fill;
            InitializeComponent();
            tabHistory.Add(homePage);
        }

        private void UnpopulateTreeNodes(TreeNodeCollection collection)
        {
            collection.Clear();
            collection.Add(new TreeNode("Loading..."));
        }

        private void PopulateTreeNodes(TreeNodeCollection collection, IEnumerable<string> names)
        {
            fdbDirectoryView.BeginUpdate();
            collection.Clear();
            foreach (var name in names)
            {
                var node = new TreeNode(name);
                node.Tag = name;
                UnpopulateTreeNodes(node.Nodes);
                collection.Add(node);
            }
            fdbDirectoryView.EndUpdate();
        }

        public async void PerformRefresh()
        {
            if (tabControl.SelectedTab == homePage)
            {
                var roots = await MainForm.PerformAsync<List<string>>("Finding roots", continuation =>
                {
                    AgentdbAdmin.ListRoots(connectionHandle, continuation);
                });

                rootList.BeginUpdate();
                var selectedRoot = (rootList.SelectedItem as RootItem)?.Root;
                RootItem itemToSelect = null;
                rootList.Items.Clear();
                foreach (var root in roots)
                {
                    var item = new RootItem() { Root = root };
                    rootList.Items.Add(item);
                    if (selectedRoot != null && root.SequenceEqual(selectedRoot))
                    {
                        itemToSelect = item;
                    }
                }
                if (itemToSelect != null)
                {
                    rootList.SelectedItem = itemToSelect;
                }
                rootList.EndUpdate();

                var dirs = await MainForm.PerformAsync<List<string>>("Listing directories", continuation =>
                {
                    AgentdbAdmin.ListDirectory(connectionHandle, new string[] { }, continuation);
                });

                PopulateTreeNodes(fdbDirectoryView.Nodes, dirs);
            } else
            {
                var viewTab = tabControl.SelectedTab?.Controls[0] as IViewTab;
                if (viewTab != null)
                {
                    viewTab.PerformRefresh();
                }
            }
        }

        private void ConnectionTab_Load(object sender, EventArgs e)
        {
            PerformRefresh();
        }

        public void OpenPage(PageId pageId)
        {
            TabPage page;
            if (!tabPages.TryGetValue(pageId, out page))
            {
                page = new TabPage(pageId.Title);
                page.Tag = pageId;

                var control = pageId.CreateTab(this, connectionHandle);
                page.Controls.Add((Control)control);

                tabPages.Add(pageId, page);
                tabControl.TabPages.Add(page);
            }
            tabControl.SelectedTab = page;
        }

        private void rootList_MouseDoubleClick(object sender, MouseEventArgs e)
        {
            var index = rootList.IndexFromPoint(e.Location);
            if (e.Button == MouseButtons.Left && index != ListBox.NoMatches)
            {
                var root = ((RootItem)rootList.Items[index]).Root;
                OpenPage(new RootPageId() { Root = root });
            }
        }

        private void rootList_KeyDown(object sender, KeyEventArgs e)
        {
            if (e.KeyCode == Keys.Enter && rootList.SelectedItem != null)
            {
                OpenPage(new RootPageId() { Root = ((RootItem)rootList.SelectedItem).Root });
            }
        }

        private void tabControl_SelectedIndexChanged(object sender, EventArgs e)
        {
            closeTabButton.Visible = tabControl.SelectedTab != null && tabControl.SelectedTab != homePage;
            tabHistory.Remove(tabControl.SelectedTab);
            tabHistory.Add(tabControl.SelectedTab);
        }

        private void closeTabButton_Click(object sender, EventArgs e)
        {
            var pageId = tabControl.SelectedTab?.Tag as PageId;
            if (pageId != null)
            {
                tabHistory.Remove(tabControl.SelectedTab);
                tabPages.Remove(pageId);
                var desiredTab = tabHistory.LastOrDefault();
                tabControl.SuspendLayout();
                closingTab = true;
                tabControl.TabPages.Remove(tabControl.SelectedTab);
                closingTab = false;
                if (desiredTab != null)
                {
                    tabControl.SelectedTab = desiredTab;
                }
                tabControl.ResumeLayout();
            }
        }

        private void tabControl_Deselecting(object sender, TabControlCancelEventArgs e)
        {
            if (closingTab)
            {
                e.Cancel = true;
            }
        }

        private async void fdbDirectoryView_BeforeExpand(object sender, TreeViewCancelEventArgs e)
        {
            var path = new List<string>();
            var node = e.Node;
            while (node != null)
            {
                path.Add(node.Text);
                node = node.Parent;
            }
            path.Reverse();

            var dirs = await MainForm.PerformAsync<List<string>>("Listing subdirectories", continuation =>
            {
                AgentdbAdmin.ListDirectory(connectionHandle, path, continuation);
            });

            PopulateTreeNodes(e.Node.Nodes, dirs);
        }

        private void fdbDirectoryView_AfterCollapse(object sender, TreeViewEventArgs e)
        {
            UnpopulateTreeNodes(e.Node.Nodes);
        }
    }

}
