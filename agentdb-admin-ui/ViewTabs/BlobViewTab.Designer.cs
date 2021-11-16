
namespace AgentdbAdmin
{
    partial class BlobViewTab
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

        #region Component Designer generated code

        /// <summary> 
        /// Required method for Designer support - do not modify 
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.tabControl = new System.Windows.Forms.TabControl();
            this.binaryPage = new System.Windows.Forms.TabPage();
            this.textPage = new System.Windows.Forms.TabPage();
            this.jsonPage = new System.Windows.Forms.TabPage();
            this.binaryTextBox = new System.Windows.Forms.TextBox();
            this.textTextBox = new System.Windows.Forms.TextBox();
            this.jsonTextBox = new System.Windows.Forms.TextBox();
            this.doesNotExistLabel = new System.Windows.Forms.Label();
            this.tabControl.SuspendLayout();
            this.binaryPage.SuspendLayout();
            this.textPage.SuspendLayout();
            this.jsonPage.SuspendLayout();
            this.SuspendLayout();
            // 
            // tabControl
            // 
            this.tabControl.Controls.Add(this.binaryPage);
            this.tabControl.Controls.Add(this.textPage);
            this.tabControl.Controls.Add(this.jsonPage);
            this.tabControl.Dock = System.Windows.Forms.DockStyle.Fill;
            this.tabControl.Location = new System.Drawing.Point(0, 0);
            this.tabControl.Margin = new System.Windows.Forms.Padding(2, 2, 2, 2);
            this.tabControl.Name = "tabControl";
            this.tabControl.SelectedIndex = 0;
            this.tabControl.Size = new System.Drawing.Size(815, 550);
            this.tabControl.TabIndex = 0;
            // 
            // binaryPage
            // 
            this.binaryPage.Controls.Add(this.binaryTextBox);
            this.binaryPage.Location = new System.Drawing.Point(4, 29);
            this.binaryPage.Margin = new System.Windows.Forms.Padding(2, 2, 2, 2);
            this.binaryPage.Name = "binaryPage";
            this.binaryPage.Padding = new System.Windows.Forms.Padding(2, 2, 2, 2);
            this.binaryPage.Size = new System.Drawing.Size(807, 517);
            this.binaryPage.TabIndex = 0;
            this.binaryPage.Text = "Binary";
            this.binaryPage.UseVisualStyleBackColor = true;
            // 
            // textPage
            // 
            this.textPage.Controls.Add(this.textTextBox);
            this.textPage.Location = new System.Drawing.Point(4, 29);
            this.textPage.Margin = new System.Windows.Forms.Padding(2, 2, 2, 2);
            this.textPage.Name = "textPage";
            this.textPage.Padding = new System.Windows.Forms.Padding(2, 2, 2, 2);
            this.textPage.Size = new System.Drawing.Size(807, 517);
            this.textPage.TabIndex = 1;
            this.textPage.Text = "Text";
            this.textPage.UseVisualStyleBackColor = true;
            // 
            // jsonPage
            // 
            this.jsonPage.Controls.Add(this.jsonTextBox);
            this.jsonPage.Location = new System.Drawing.Point(4, 29);
            this.jsonPage.Name = "jsonPage";
            this.jsonPage.Padding = new System.Windows.Forms.Padding(3);
            this.jsonPage.Size = new System.Drawing.Size(807, 517);
            this.jsonPage.TabIndex = 2;
            this.jsonPage.Text = "JSON";
            this.jsonPage.UseVisualStyleBackColor = true;
            // 
            // binaryTextBox
            // 
            this.binaryTextBox.BackColor = System.Drawing.SystemColors.Window;
            this.binaryTextBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.binaryTextBox.Font = new System.Drawing.Font("Consolas", 9.75F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.binaryTextBox.Location = new System.Drawing.Point(2, 2);
            this.binaryTextBox.Multiline = true;
            this.binaryTextBox.Name = "binaryTextBox";
            this.binaryTextBox.ReadOnly = true;
            this.binaryTextBox.Size = new System.Drawing.Size(803, 513);
            this.binaryTextBox.TabIndex = 0;
            // 
            // textTextBox
            // 
            this.textTextBox.BackColor = System.Drawing.SystemColors.Window;
            this.textTextBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.textTextBox.Font = new System.Drawing.Font("Consolas", 9.75F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.textTextBox.Location = new System.Drawing.Point(2, 2);
            this.textTextBox.Multiline = true;
            this.textTextBox.Name = "textTextBox";
            this.textTextBox.ReadOnly = true;
            this.textTextBox.Size = new System.Drawing.Size(803, 513);
            this.textTextBox.TabIndex = 1;
            // 
            // jsonTextBox
            // 
            this.jsonTextBox.BackColor = System.Drawing.SystemColors.Window;
            this.jsonTextBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.jsonTextBox.Font = new System.Drawing.Font("Consolas", 9.75F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.jsonTextBox.Location = new System.Drawing.Point(3, 3);
            this.jsonTextBox.Multiline = true;
            this.jsonTextBox.Name = "jsonTextBox";
            this.jsonTextBox.ReadOnly = true;
            this.jsonTextBox.Size = new System.Drawing.Size(801, 511);
            this.jsonTextBox.TabIndex = 1;
            // 
            // doesNotExistLabel
            // 
            this.doesNotExistLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.doesNotExistLabel.Location = new System.Drawing.Point(0, 0);
            this.doesNotExistLabel.Name = "doesNotExistLabel";
            this.doesNotExistLabel.Size = new System.Drawing.Size(815, 550);
            this.doesNotExistLabel.TabIndex = 1;
            this.doesNotExistLabel.Text = "Blob does not exist!";
            this.doesNotExistLabel.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            this.doesNotExistLabel.Visible = false;
            // 
            // BlobViewTab
            // 
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.None;
            this.Controls.Add(this.tabControl);
            this.Controls.Add(this.doesNotExistLabel);
            this.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.Margin = new System.Windows.Forms.Padding(2, 2, 2, 2);
            this.Name = "BlobViewTab";
            this.Size = new System.Drawing.Size(815, 550);
            this.tabControl.ResumeLayout(false);
            this.binaryPage.ResumeLayout(false);
            this.binaryPage.PerformLayout();
            this.textPage.ResumeLayout(false);
            this.textPage.PerformLayout();
            this.jsonPage.ResumeLayout(false);
            this.jsonPage.PerformLayout();
            this.ResumeLayout(false);

        }

        #endregion

        private System.Windows.Forms.TabControl tabControl;
        private System.Windows.Forms.TabPage binaryPage;
        private System.Windows.Forms.TextBox binaryTextBox;
        private System.Windows.Forms.TabPage textPage;
        private System.Windows.Forms.TextBox textTextBox;
        private System.Windows.Forms.TabPage jsonPage;
        private System.Windows.Forms.TextBox jsonTextBox;
        private System.Windows.Forms.Label doesNotExistLabel;
    }
}
