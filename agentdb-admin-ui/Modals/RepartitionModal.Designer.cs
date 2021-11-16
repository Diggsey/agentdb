
namespace AgentdbAdmin.Modals
{
    partial class RepartitionModal
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            System.Windows.Forms.Label recvPartitionLabel;
            System.Windows.Forms.Label sendPartitionLabel;
            System.Windows.Forms.Label detailsNameLabel;
            System.Windows.Forms.Label newPartitionRangeLabel;
            System.Windows.Forms.TableLayoutPanel newPartitionRangeTableLayout;
            System.Windows.Forms.Label newPartitionToLabel;
            System.Windows.Forms.Label oldPartitionCountLabel;
            System.Windows.Forms.Label newPartitionCountLabel;
            this.detailsTableLayout = new System.Windows.Forms.TableLayoutPanel();
            this.recvPartitionBox = new System.Windows.Forms.TextBox();
            this.sendPartitionBox = new System.Windows.Forms.TextBox();
            this.detailsNameBox = new System.Windows.Forms.TextBox();
            this.newPartitionFromBox = new System.Windows.Forms.NumericUpDown();
            this.newPartitionToBox = new System.Windows.Forms.NumericUpDown();
            this.modalButtonsFlowLayout = new System.Windows.Forms.FlowLayoutPanel();
            this.cancelButton = new System.Windows.Forms.Button();
            this.acceptButton = new System.Windows.Forms.Button();
            this.oldPartitionCountBox = new System.Windows.Forms.TextBox();
            this.newPartitionCountBox = new System.Windows.Forms.TextBox();
            this.errorBox = new System.Windows.Forms.TextBox();
            recvPartitionLabel = new System.Windows.Forms.Label();
            sendPartitionLabel = new System.Windows.Forms.Label();
            detailsNameLabel = new System.Windows.Forms.Label();
            newPartitionRangeLabel = new System.Windows.Forms.Label();
            newPartitionRangeTableLayout = new System.Windows.Forms.TableLayoutPanel();
            newPartitionToLabel = new System.Windows.Forms.Label();
            oldPartitionCountLabel = new System.Windows.Forms.Label();
            newPartitionCountLabel = new System.Windows.Forms.Label();
            this.detailsTableLayout.SuspendLayout();
            newPartitionRangeTableLayout.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.newPartitionFromBox)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.newPartitionToBox)).BeginInit();
            this.modalButtonsFlowLayout.SuspendLayout();
            this.SuspendLayout();
            // 
            // detailsTableLayout
            // 
            this.detailsTableLayout.AutoSize = true;
            this.detailsTableLayout.AutoSizeMode = System.Windows.Forms.AutoSizeMode.GrowAndShrink;
            this.detailsTableLayout.ColumnCount = 2;
            this.detailsTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            this.detailsTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
            this.detailsTableLayout.Controls.Add(this.errorBox, 0, 6);
            this.detailsTableLayout.Controls.Add(this.newPartitionCountBox, 1, 5);
            this.detailsTableLayout.Controls.Add(this.oldPartitionCountBox, 1, 4);
            this.detailsTableLayout.Controls.Add(newPartitionCountLabel, 0, 5);
            this.detailsTableLayout.Controls.Add(oldPartitionCountLabel, 0, 4);
            this.detailsTableLayout.Controls.Add(newPartitionRangeLabel, 0, 3);
            this.detailsTableLayout.Controls.Add(this.recvPartitionBox, 1, 2);
            this.detailsTableLayout.Controls.Add(recvPartitionLabel, 0, 2);
            this.detailsTableLayout.Controls.Add(this.sendPartitionBox, 1, 1);
            this.detailsTableLayout.Controls.Add(sendPartitionLabel, 0, 1);
            this.detailsTableLayout.Controls.Add(detailsNameLabel, 0, 0);
            this.detailsTableLayout.Controls.Add(this.detailsNameBox, 1, 0);
            this.detailsTableLayout.Controls.Add(newPartitionRangeTableLayout, 1, 3);
            this.detailsTableLayout.Dock = System.Windows.Forms.DockStyle.Fill;
            this.detailsTableLayout.Location = new System.Drawing.Point(10, 10);
            this.detailsTableLayout.Name = "detailsTableLayout";
            this.detailsTableLayout.RowCount = 7;
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 100F));
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.detailsTableLayout.Size = new System.Drawing.Size(420, 253);
            this.detailsTableLayout.TabIndex = 1;
            // 
            // recvPartitionBox
            // 
            this.recvPartitionBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.recvPartitionBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.recvPartitionBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.recvPartitionBox.Location = new System.Drawing.Point(182, 48);
            this.recvPartitionBox.Margin = new System.Windows.Forms.Padding(2, 2, 2, 2);
            this.recvPartitionBox.Name = "recvPartitionBox";
            this.recvPartitionBox.ReadOnly = true;
            this.recvPartitionBox.Size = new System.Drawing.Size(236, 19);
            this.recvPartitionBox.TabIndex = 9;
            // 
            // recvPartitionLabel
            // 
            recvPartitionLabel.AutoSize = true;
            recvPartitionLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            recvPartitionLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            recvPartitionLabel.Location = new System.Drawing.Point(2, 46);
            recvPartitionLabel.Margin = new System.Windows.Forms.Padding(2, 0, 2, 0);
            recvPartitionLabel.Name = "recvPartitionLabel";
            recvPartitionLabel.Size = new System.Drawing.Size(176, 23);
            recvPartitionLabel.TabIndex = 8;
            recvPartitionLabel.Text = "Receive partition range:";
            recvPartitionLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // sendPartitionBox
            // 
            this.sendPartitionBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.sendPartitionBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.sendPartitionBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.sendPartitionBox.Location = new System.Drawing.Point(182, 25);
            this.sendPartitionBox.Margin = new System.Windows.Forms.Padding(2, 2, 2, 2);
            this.sendPartitionBox.Name = "sendPartitionBox";
            this.sendPartitionBox.ReadOnly = true;
            this.sendPartitionBox.Size = new System.Drawing.Size(236, 19);
            this.sendPartitionBox.TabIndex = 7;
            // 
            // sendPartitionLabel
            // 
            sendPartitionLabel.AutoSize = true;
            sendPartitionLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            sendPartitionLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            sendPartitionLabel.Location = new System.Drawing.Point(2, 23);
            sendPartitionLabel.Margin = new System.Windows.Forms.Padding(2, 0, 2, 0);
            sendPartitionLabel.Name = "sendPartitionLabel";
            sendPartitionLabel.Size = new System.Drawing.Size(176, 23);
            sendPartitionLabel.TabIndex = 6;
            sendPartitionLabel.Text = "Send partition range:";
            sendPartitionLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // detailsNameLabel
            // 
            detailsNameLabel.AutoSize = true;
            detailsNameLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            detailsNameLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            detailsNameLabel.Location = new System.Drawing.Point(2, 0);
            detailsNameLabel.Margin = new System.Windows.Forms.Padding(2, 0, 2, 0);
            detailsNameLabel.Name = "detailsNameLabel";
            detailsNameLabel.Size = new System.Drawing.Size(176, 23);
            detailsNameLabel.TabIndex = 0;
            detailsNameLabel.Text = "Name:";
            detailsNameLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // detailsNameBox
            // 
            this.detailsNameBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.detailsNameBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.detailsNameBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.detailsNameBox.Location = new System.Drawing.Point(182, 2);
            this.detailsNameBox.Margin = new System.Windows.Forms.Padding(2, 2, 2, 2);
            this.detailsNameBox.Name = "detailsNameBox";
            this.detailsNameBox.ReadOnly = true;
            this.detailsNameBox.Size = new System.Drawing.Size(236, 19);
            this.detailsNameBox.TabIndex = 1;
            // 
            // newPartitionRangeLabel
            // 
            newPartitionRangeLabel.AutoSize = true;
            newPartitionRangeLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            newPartitionRangeLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            newPartitionRangeLabel.Location = new System.Drawing.Point(2, 69);
            newPartitionRangeLabel.Margin = new System.Windows.Forms.Padding(2, 0, 2, 0);
            newPartitionRangeLabel.Name = "newPartitionRangeLabel";
            newPartitionRangeLabel.Size = new System.Drawing.Size(176, 34);
            newPartitionRangeLabel.TabIndex = 10;
            newPartitionRangeLabel.Text = "New partition range:";
            newPartitionRangeLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // newPartitionRangeTableLayout
            // 
            newPartitionRangeTableLayout.AutoSize = true;
            newPartitionRangeTableLayout.AutoSizeMode = System.Windows.Forms.AutoSizeMode.GrowAndShrink;
            newPartitionRangeTableLayout.ColumnCount = 4;
            newPartitionRangeTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            newPartitionRangeTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            newPartitionRangeTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            newPartitionRangeTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            newPartitionRangeTableLayout.Controls.Add(this.newPartitionToBox, 2, 0);
            newPartitionRangeTableLayout.Controls.Add(this.newPartitionFromBox, 0, 0);
            newPartitionRangeTableLayout.Controls.Add(newPartitionToLabel, 1, 0);
            newPartitionRangeTableLayout.Dock = System.Windows.Forms.DockStyle.Fill;
            newPartitionRangeTableLayout.Location = new System.Drawing.Point(182, 71);
            newPartitionRangeTableLayout.Margin = new System.Windows.Forms.Padding(2, 2, 2, 2);
            newPartitionRangeTableLayout.Name = "newPartitionRangeTableLayout";
            newPartitionRangeTableLayout.RowCount = 1;
            newPartitionRangeTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 100F));
            newPartitionRangeTableLayout.Size = new System.Drawing.Size(236, 30);
            newPartitionRangeTableLayout.TabIndex = 12;
            // 
            // newPartitionFromBox
            // 
            this.newPartitionFromBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.newPartitionFromBox.Location = new System.Drawing.Point(2, 2);
            this.newPartitionFromBox.Margin = new System.Windows.Forms.Padding(2, 2, 2, 2);
            this.newPartitionFromBox.Maximum = new decimal(new int[] {
            100000,
            0,
            0,
            0});
            this.newPartitionFromBox.Name = "newPartitionFromBox";
            this.newPartitionFromBox.Size = new System.Drawing.Size(100, 26);
            this.newPartitionFromBox.TabIndex = 0;
            this.newPartitionFromBox.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
            this.newPartitionFromBox.ValueChanged += new System.EventHandler(this.newPartitionFromBox_ValueChanged);
            // 
            // newPartitionToLabel
            // 
            newPartitionToLabel.AutoSize = true;
            newPartitionToLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            newPartitionToLabel.Location = new System.Drawing.Point(107, 0);
            newPartitionToLabel.Name = "newPartitionToLabel";
            newPartitionToLabel.Size = new System.Drawing.Size(23, 30);
            newPartitionToLabel.TabIndex = 1;
            newPartitionToLabel.Text = "to";
            newPartitionToLabel.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // newPartitionToBox
            // 
            this.newPartitionToBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.newPartitionToBox.Location = new System.Drawing.Point(135, 2);
            this.newPartitionToBox.Margin = new System.Windows.Forms.Padding(2);
            this.newPartitionToBox.Maximum = new decimal(new int[] {
            100000,
            0,
            0,
            0});
            this.newPartitionToBox.Name = "newPartitionToBox";
            this.newPartitionToBox.Size = new System.Drawing.Size(100, 26);
            this.newPartitionToBox.TabIndex = 2;
            this.newPartitionToBox.ValueChanged += new System.EventHandler(this.newPartitionToBox_ValueChanged);
            // 
            // oldPartitionCountLabel
            // 
            oldPartitionCountLabel.AutoSize = true;
            oldPartitionCountLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            oldPartitionCountLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            oldPartitionCountLabel.Location = new System.Drawing.Point(2, 103);
            oldPartitionCountLabel.Margin = new System.Windows.Forms.Padding(2, 0, 2, 0);
            oldPartitionCountLabel.Name = "oldPartitionCountLabel";
            oldPartitionCountLabel.Size = new System.Drawing.Size(176, 23);
            oldPartitionCountLabel.TabIndex = 14;
            oldPartitionCountLabel.Text = "Old partition count:";
            oldPartitionCountLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // newPartitionCountLabel
            // 
            newPartitionCountLabel.AutoSize = true;
            newPartitionCountLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            newPartitionCountLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            newPartitionCountLabel.Location = new System.Drawing.Point(2, 126);
            newPartitionCountLabel.Margin = new System.Windows.Forms.Padding(2, 0, 2, 0);
            newPartitionCountLabel.Name = "newPartitionCountLabel";
            newPartitionCountLabel.Size = new System.Drawing.Size(176, 23);
            newPartitionCountLabel.TabIndex = 15;
            newPartitionCountLabel.Text = "New partition count:";
            newPartitionCountLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // modalButtonsFlowLayout
            // 
            this.modalButtonsFlowLayout.AutoSize = true;
            this.modalButtonsFlowLayout.Controls.Add(this.cancelButton);
            this.modalButtonsFlowLayout.Controls.Add(this.acceptButton);
            this.modalButtonsFlowLayout.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.modalButtonsFlowLayout.FlowDirection = System.Windows.Forms.FlowDirection.RightToLeft;
            this.modalButtonsFlowLayout.Location = new System.Drawing.Point(10, 263);
            this.modalButtonsFlowLayout.Name = "modalButtonsFlowLayout";
            this.modalButtonsFlowLayout.Size = new System.Drawing.Size(420, 36);
            this.modalButtonsFlowLayout.TabIndex = 2;
            // 
            // cancelButton
            // 
            this.cancelButton.AutoSize = true;
            this.cancelButton.DialogResult = System.Windows.Forms.DialogResult.Cancel;
            this.cancelButton.Location = new System.Drawing.Point(342, 3);
            this.cancelButton.Name = "cancelButton";
            this.cancelButton.Size = new System.Drawing.Size(75, 30);
            this.cancelButton.TabIndex = 0;
            this.cancelButton.Text = "Cancel";
            this.cancelButton.UseVisualStyleBackColor = true;
            // 
            // acceptButton
            // 
            this.acceptButton.AutoSize = true;
            this.acceptButton.Location = new System.Drawing.Point(234, 3);
            this.acceptButton.Name = "acceptButton";
            this.acceptButton.Size = new System.Drawing.Size(102, 30);
            this.acceptButton.TabIndex = 1;
            this.acceptButton.Text = "Re-partition";
            this.acceptButton.UseVisualStyleBackColor = true;
            this.acceptButton.Click += new System.EventHandler(this.acceptButton_Click);
            // 
            // oldPartitionCountBox
            // 
            this.oldPartitionCountBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.oldPartitionCountBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.oldPartitionCountBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.oldPartitionCountBox.Location = new System.Drawing.Point(182, 105);
            this.oldPartitionCountBox.Margin = new System.Windows.Forms.Padding(2);
            this.oldPartitionCountBox.Name = "oldPartitionCountBox";
            this.oldPartitionCountBox.ReadOnly = true;
            this.oldPartitionCountBox.Size = new System.Drawing.Size(236, 19);
            this.oldPartitionCountBox.TabIndex = 16;
            // 
            // newPartitionCountBox
            // 
            this.newPartitionCountBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.newPartitionCountBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.newPartitionCountBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.newPartitionCountBox.Location = new System.Drawing.Point(182, 128);
            this.newPartitionCountBox.Margin = new System.Windows.Forms.Padding(2);
            this.newPartitionCountBox.Name = "newPartitionCountBox";
            this.newPartitionCountBox.ReadOnly = true;
            this.newPartitionCountBox.Size = new System.Drawing.Size(236, 19);
            this.newPartitionCountBox.TabIndex = 17;
            // 
            // errorBox
            // 
            this.errorBox.BackColor = System.Drawing.SystemColors.Control;
            this.errorBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.detailsTableLayout.SetColumnSpan(this.errorBox, 2);
            this.errorBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.errorBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.errorBox.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(192)))), ((int)(((byte)(0)))), ((int)(((byte)(0)))));
            this.errorBox.Location = new System.Drawing.Point(15, 164);
            this.errorBox.Margin = new System.Windows.Forms.Padding(15);
            this.errorBox.Multiline = true;
            this.errorBox.Name = "errorBox";
            this.errorBox.ReadOnly = true;
            this.errorBox.Size = new System.Drawing.Size(390, 74);
            this.errorBox.TabIndex = 18;
            this.errorBox.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
            // 
            // RepartitionModal
            // 
            this.AcceptButton = this.acceptButton;
            this.AutoScaleDimensions = new System.Drawing.SizeF(9F, 20F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.AutoSize = true;
            this.AutoSizeMode = System.Windows.Forms.AutoSizeMode.GrowAndShrink;
            this.CancelButton = this.cancelButton;
            this.ClientSize = new System.Drawing.Size(440, 309);
            this.Controls.Add(this.detailsTableLayout);
            this.Controls.Add(this.modalButtonsFlowLayout);
            this.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedDialog;
            this.Margin = new System.Windows.Forms.Padding(2, 2, 2, 2);
            this.MaximizeBox = false;
            this.MinimizeBox = false;
            this.Name = "RepartitionModal";
            this.Padding = new System.Windows.Forms.Padding(10);
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
            this.Text = "Re-partition AgentDB Root";
            this.detailsTableLayout.ResumeLayout(false);
            this.detailsTableLayout.PerformLayout();
            newPartitionRangeTableLayout.ResumeLayout(false);
            newPartitionRangeTableLayout.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.newPartitionFromBox)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.newPartitionToBox)).EndInit();
            this.modalButtonsFlowLayout.ResumeLayout(false);
            this.modalButtonsFlowLayout.PerformLayout();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.TableLayoutPanel detailsTableLayout;
        private System.Windows.Forms.TextBox recvPartitionBox;
        private System.Windows.Forms.TextBox sendPartitionBox;
        private System.Windows.Forms.TextBox detailsNameBox;
        private System.Windows.Forms.NumericUpDown newPartitionToBox;
        private System.Windows.Forms.NumericUpDown newPartitionFromBox;
        private System.Windows.Forms.FlowLayoutPanel modalButtonsFlowLayout;
        private System.Windows.Forms.Button cancelButton;
        private System.Windows.Forms.Button acceptButton;
        private System.Windows.Forms.TextBox newPartitionCountBox;
        private System.Windows.Forms.TextBox oldPartitionCountBox;
        private System.Windows.Forms.TextBox errorBox;
    }
}