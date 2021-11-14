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
    public partial class BlobViewTab : UserControl, IViewTab
    {
        private ConnectionTab parent;
        private AgentdbAdmin.IOpaqueHandle connectionHandle;
        private List<byte> root;
        private Guid blobId;
        private byte[] blobData;

        public BlobViewTab(ConnectionTab parent, AgentdbAdmin.IOpaqueHandle connectionHandle, List<byte> root, Guid blobId)
        {
            this.parent = parent;
            this.connectionHandle = connectionHandle;
            this.root = root;
            this.blobId = blobId;
            this.Dock = DockStyle.Fill;
            InitializeComponent();
            PerformRefresh();
        }

        public async void PerformRefresh()
        {
            blobData = (await parent.MainForm.PerformAsync<List<byte>>("Loading blob", continuation =>
            {
                AgentdbAdmin.LoadBlob(connectionHandle, root, blobId, continuation);
            })).ToArray();
            SuspendLayout();
            if (blobData == null)
            {
                doesNotExistLabel.Visible = true;
                tabControl.Visible = false;
            } else
            {
                doesNotExistLabel.Visible = false;
                tabControl.Visible = true;
                binaryTextBox.Text = Utils.FormatBinary(blobData);
                textTextBox.Text = Encoding.UTF8.GetString(blobData);
                jsonTextBox.Text = Utils.FormatJson(textTextBox.Text);
            }
            ResumeLayout();
        }
    }
}
