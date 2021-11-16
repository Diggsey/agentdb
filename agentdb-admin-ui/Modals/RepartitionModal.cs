using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace AgentdbAdmin.Modals
{
    public partial class RepartitionModal : Form
    {
        private (uint, uint) partitionRecvRange;
        private (uint, uint) partitionSendRange;
        private (uint, uint) newPartitionRange;
        private string rootName;

        public RepartitionModal()
        {
            InitializeComponent();
            DoUpdate();
        }

        private void DoUpdate()
        {
            SuspendLayout();
            detailsNameBox.Text = rootName;
            recvPartitionBox.Text = $"{partitionRecvRange.Item1} to {partitionRecvRange.Item2 - 1}";
            sendPartitionBox.Text = $"{partitionSendRange.Item1} to {partitionSendRange.Item2 - 1}";
            newPartitionFromBox.Value = newPartitionRange.Item1;
            newPartitionToBox.Value = Math.Max(0, (decimal)newPartitionRange.Item2 - 1);
            oldPartitionCountBox.Text = (partitionRecvRange.Item2 - partitionRecvRange.Item1).ToString();
            newPartitionCountBox.Text = (
                newPartitionRange.Item2 >= newPartitionRange.Item1
                ? newPartitionRange.Item2 - newPartitionRange.Item1
                : 0
            ).ToString();

            var errors = new List<string>();
            if (newPartitionRange.Item2 <= newPartitionRange.Item1)
            {
                errors.Add("New partition range must not be empty.");
            }
            if (newPartitionRange.Item2 > partitionRecvRange.Item1 && newPartitionRange.Item1 < partitionRecvRange.Item2)
            {
                errors.Add("The new partition range must not overlap with the previous one.");
            }
            acceptButton.Enabled = errors.Count == 0;
            errorBox.Visible = errors.Count > 0;
            errorBox.Text = string.Join(" ", errors);

            ResumeLayout();
        }

        public (uint, uint) PartitionRecvRange { get => partitionRecvRange; set { partitionRecvRange = value; DoUpdate(); } }
        public (uint, uint) PartitionSendRange { get => partitionSendRange; set { partitionSendRange = value; DoUpdate(); } }
        public (uint, uint) NewPartitionRange { get => newPartitionRange; set { newPartitionRange = value; DoUpdate(); } }
        public string RootName { get => rootName; set { rootName = value; DoUpdate(); } }

        private void newPartitionFromBox_ValueChanged(object sender, EventArgs e)
        {
            NewPartitionRange = ((uint)newPartitionFromBox.Value, newPartitionRange.Item2);
        }

        private void newPartitionToBox_ValueChanged(object sender, EventArgs e)
        {
            NewPartitionRange = (newPartitionRange.Item1, (uint)newPartitionToBox.Value + 1);
        }

        private void acceptButton_Click(object sender, EventArgs e)
        {
            DialogResult = DialogResult.OK;
            Close();
        }
    }
}
