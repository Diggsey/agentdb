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
    public partial class DirectoryViewTab : UserControl, IViewTab
    {
        private ConnectionTab parent;
        private AgentdbAdmin.IOpaqueHandle connectionHandle;
        private List<string> path;
        private List<byte> from = new List<byte>();
        private uint limit = 100;
        private bool reverse = false;
        private List<AgentdbAdmin.KeyValueDesc> keyValues = new List<AgentdbAdmin.KeyValueDesc>();

        public DirectoryViewTab(ConnectionTab parent, AgentdbAdmin.IOpaqueHandle connectionHandle, List<string> path)
        {
            this.parent = parent;
            this.connectionHandle = connectionHandle;
            this.path = path;
            this.Dock = DockStyle.Fill;
            InitializeComponent();
            PerformRefresh();

            directoryPathBox.Text = string.Join("/", path);
        }

        public async void PerformRefresh()
        {
            var dirDesc = (await parent.MainForm.PerformAsync<AgentdbAdmin.DirectoryDesc>("Opening directory", continuation =>
            {
                AgentdbAdmin.OpenDirectory(connectionHandle, path, continuation);
            }));
            keyValues = (await parent.MainForm.PerformAsync<List<AgentdbAdmin.KeyValueDesc>>("Listing subspace", continuation =>
            {
                AgentdbAdmin.ListSubspace(connectionHandle, dirDesc.prefix, from, limit, reverse, continuation);
            }));
            if (reverse)
            {
                keyValues.Reverse();
            }
            SuspendLayout();

            layerBox.Text = Utils.StringifyBytes(dirDesc.layer);
            itemsListView.BeginUpdate();
            itemsListView.Items.Clear();
            itemsListView.Columns.Clear();

            if (keyValues.Count > 0)
            {
                var maxKeyParts = keyValues.Max(kv => kv.keyDecoded.Count);
                for (var i = 0; i < maxKeyParts; ++i)
                {
                    itemsListView.Columns.Add($"Key {i}");
                }
                itemsListView.Columns.Add("Value");

                foreach (var keyValue in keyValues)
                {
                    var parts = new List<string>();
                    for (var i = 0; i < maxKeyParts; ++i)
                    {
                        parts.Add(keyValue.keyDecoded.ElementAtOrDefault(i));
                    }
                    parts.Add(Utils.StringifyBytes(keyValue.valueBytes));
                    itemsListView.Items.Add(new ListViewItem(parts.ToArray()));
                }
                itemsListView.AutoResizeColumns(ColumnHeaderAutoResizeStyle.ColumnContent);
            }
            itemsListView.EndUpdate();
            ResumeLayout();
        }

        private void agentsPerPageBox_ValueChanged(object sender, EventArgs e)
        {
            limit = (uint)itemsPerPageBox.Value;
        }

        private void nextPageButton_Click(object sender, EventArgs e)
        {
            from = (keyValues.Count > 0) ? keyValues.Last().keyBytes : new List<byte>();
            reverse = false;
            PerformRefresh();
        }

        private void prevPageButton_Click(object sender, EventArgs e)
        {
            from = (keyValues.Count > 0) ? keyValues.First().keyBytes : new List<byte>();
            reverse = true;
            PerformRefresh();
        }
    }
}
