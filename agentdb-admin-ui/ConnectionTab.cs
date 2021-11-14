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
            public List<byte> Root { get; set; }
            public string Title
            {
                get
                {
                    return Utils.StringifyBytes(Root);
                }
            }
        }

        public abstract class PageId
        {
            public abstract string Title { get;  }
            public abstract IViewTab CreateTab(ConnectionTab parent, AgentdbAdmin.IOpaqueHandle connectionHandle);
        }

        public class RootPageId: PageId
        {
            public List<byte> Root { get; set; }
            public override string Title => Utils.StringifyBytes(Root);

            public override IViewTab CreateTab(ConnectionTab parent, AgentdbAdmin.IOpaqueHandle connectionHandle)
            {
                return new RootViewTab(parent, connectionHandle, Root);
            }

            public override bool Equals(object obj)
            {
                return obj is RootPageId id &&
                       EqualityComparer<List<byte>>.Default.Equals(Root, id.Root);
            }

            public override int GetHashCode()
            {
                return -1490287827 + EqualityComparer<List<byte>>.Default.GetHashCode(Root);
            }
        }

        public class BlobPageId : PageId
        {
            public List<byte> Root { get; set; }
            public Guid BlobId { get; set; }

            public override string Title => Utils.StringifyBytes(Root) + ": " + BlobId.ToString();

            public override IViewTab CreateTab(ConnectionTab parent, AgentdbAdmin.IOpaqueHandle connectionHandle)
            {
                return new BlobViewTab(parent, connectionHandle, Root, BlobId);
            }

            public override bool Equals(object obj)
            {
                return obj is BlobPageId id &&
                       EqualityComparer<List<byte>>.Default.Equals(Root, id.Root) &&
                       BlobId.Equals(id.BlobId);
            }

            public override int GetHashCode()
            {
                int hashCode = 1616903828;
                hashCode = hashCode * -1521134295 + EqualityComparer<List<byte>>.Default.GetHashCode(Root);
                hashCode = hashCode * -1521134295 + BlobId.GetHashCode();
                return hashCode;
            }
        }

        public MainForm MainForm { get; set; }
        private AgentdbAdmin.IOpaqueHandle connectionHandle;
        private Dictionary<PageId, TabPage> tabPages = new Dictionary<PageId, TabPage>();

        public ConnectionTab(MainForm parent, AgentdbAdmin.IOpaqueHandle connectionHandle)
        {
            this.MainForm = parent;
            this.connectionHandle = connectionHandle;
            this.Dock = DockStyle.Fill;
            InitializeComponent();
        }
        public async void PerformRefresh()
        {
            if (tabControl.SelectedTab == homePage)
            {
                var roots = await MainForm.PerformAsync<List<List<byte>>>("Finding roots", continuation =>
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
        }

        private void closeTabButton_Click(object sender, EventArgs e)
        {
            var pageId = tabControl.SelectedTab?.Tag as PageId;
            if (pageId != null)
            {
                tabPages.Remove(pageId);
                tabControl.TabPages.Remove(tabControl.SelectedTab);
            }
        }
    }

}
