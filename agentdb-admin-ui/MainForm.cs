using AgentdbAdmin.Properties;
using System;
using System.Collections.Generic;
using System.Collections.Specialized;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace AgentdbAdmin
{
    public partial class MainForm : Form
    {
        private List<string> backgroundTasks = new List<string>();
        public MainForm()
        {
            InitializeComponent();

            switch (Environment.OSVersion.Platform)
            {
                case PlatformID.Win32NT:
                    openFileDialog.InitialDirectory = "C:\\ProgramData\\foundationdb\\";
                    break;
                case PlatformID.MacOSX:
                    openFileDialog.InitialDirectory = "/usr/local/etc/foundationdb/";
                    break;
                default:
                    openFileDialog.InitialDirectory = "/etc/foundationdb/fdb.cluster";
                    break;
            }
        }

        private ConnectionTab ActiveTab
        {
            get
            {
                return (ConnectionTab)connectionTabs.SelectedTab?.Controls[0];
            }
        }

        private void UpdateStatus()
        {
            SuspendLayout();
            if (backgroundTasks.Count == 0)
            {
                statusLabel.Text = "Idle";
                statusProgressBar.Visible = false;
            } else
            {
                statusLabel.Text = string.Join(", ", backgroundTasks) + "...";
                statusProgressBar.Visible = true;
            }
            ResumeLayout();
            UpdateEnablement();
        }

        private void UpdateEnablement()
        {
            SuspendLayout();
            bool actionsVisible = connectionTabs.TabPages.Count > 0;
            bool actionsEnabled = backgroundTasks.Count == 0 && actionsVisible;
            toolStripSeparator.Visible = actionsVisible;
            refreshButton.Enabled = actionsEnabled;
            refreshButton.Visible = actionsVisible;
            closeTabButton.Visible = actionsVisible;
            autoRefreshLabel.Visible = actionsVisible;
            autoRefreshBox.Visible = actionsVisible;
            ResumeLayout();
        }

        public Task<T> PerformAsync<T>(string name, Action<Action<T, string>> action)
        {
            backgroundTasks.Add(name);
            UpdateStatus();
            var task = new TaskCompletionSource<T>();
            action((res, err) => this.Invoke(new Action(() => {
                backgroundTasks.Remove(name);
                UpdateStatus();
                if (err != null)
                {
                    task.SetException(new AgentdbAdmin.RustException(err));
                } else
                {
                    task.SetResult(res);
                }
            })));
            return task.Task;
        }

        private bool OpenConnection(string path)
        {
            AgentdbAdmin.IOpaqueHandle connectionHandle;
            try
            {
                connectionHandle = AgentdbAdmin.Connect(path);
            }
            catch (AgentdbAdmin.RustException ex)
            {
                MessageBox.Show(this, ex.Message, "Failed to connect", MessageBoxButtons.OK, MessageBoxIcon.Error);
                return false;
            }
            string name;
            if (path == null)
            {
                name = "Default";
            } else
            {
                name = Path.GetFileName(Path.GetDirectoryName(path)) + "/" + Path.GetFileName(path);
            }
            var page = new TabPage(name);
            page.Controls.Add(new ConnectionTab(this, connectionHandle));
            connectionTabs.TabPages.Add(page);
            UpdateEnablement();
            return true;
        }

        private StringCollection RecentlyOpenedFiles
        {
            get
            {
                if (Settings.Default.RecentlyOpenedFiles == null)
                {
                    Settings.Default.RecentlyOpenedFiles = new StringCollection();
                }
                return Settings.Default.RecentlyOpenedFiles;
            }
        }

        private void newConnectionButton_Click(object sender, EventArgs e)
        {
            OpenConnection(null);
        }

        private void openConnectionButton_DropDownOpening(object sender, EventArgs e)
        {
            openConnectionButton.DropDownItems.Clear();
            if (RecentlyOpenedFiles.Count > 0)
            {
                foreach (var filename in RecentlyOpenedFiles)
                {
                    openConnectionButton.DropDownItems.Add(filename);
                }
            } else
            {
                var item = new ToolStripMenuItem("No recent files...");
                item.Enabled = false;
                openConnectionButton.DropDownItems.Add(item);
            }
        }

        private void openConnectionButton_ButtonClick(object sender, EventArgs e)
        {
            if (openFileDialog.ShowDialog() == DialogResult.OK)
            {
                if (OpenConnection(openFileDialog.FileName))
                {
                    RecentlyOpenedFiles.Insert(0, openFileDialog.FileName);
                    while (RecentlyOpenedFiles.Count > 10)
                    {
                        RecentlyOpenedFiles.RemoveAt(10);
                    }
                    Settings.Default.Save();
                }
            }
        }

        private void openConnectionButton_DropDownItemClicked(object sender, ToolStripItemClickedEventArgs e)
        {
            if (OpenConnection(e.ClickedItem.Text))
            {
                RecentlyOpenedFiles.Remove(e.ClickedItem.Text);
                RecentlyOpenedFiles.Insert(0, e.ClickedItem.Text);
                Settings.Default.Save();
            }
        }

        private void closeTabButton_Click(object sender, EventArgs e)
        {
            connectionTabs.TabPages.RemoveAt(connectionTabs.SelectedIndex);
            UpdateEnablement();
        }

        private void refreshButton_Click(object sender, EventArgs e)
        {
            ActiveTab.PerformRefresh();
        }

        private void autoRefreshTimer_Tick(object sender, EventArgs e)
        {
            ActiveTab.PerformRefresh();
        }

        private void autoRefreshBox_SelectedIndexChanged(object sender, EventArgs e)
        {
            switch (autoRefreshBox.SelectedIndex)
            {
                case 0:
                    autoRefreshTimer.Enabled = false;
                    break;
                case 1:
                    autoRefreshTimer.Interval = 1000;
                    autoRefreshTimer.Enabled = true;
                    break;
                case 2:
                    autoRefreshTimer.Interval = 5000;
                    autoRefreshTimer.Enabled = true;
                    break;
                case 3:
                    autoRefreshTimer.Interval = 30000;
                    autoRefreshTimer.Enabled = true;
                    break;
            }
        }
    }
}
