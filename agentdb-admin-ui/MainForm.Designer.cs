
namespace AgentdbAdmin
{
    partial class MainForm
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
            this.components = new System.ComponentModel.Container();
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(MainForm));
            this.toolStrip = new System.Windows.Forms.ToolStrip();
            this.newConnectionButton = new System.Windows.Forms.ToolStripButton();
            this.openConnectionButton = new System.Windows.Forms.ToolStripSplitButton();
            this.toolStripSeparator = new System.Windows.Forms.ToolStripSeparator();
            this.refreshButton = new System.Windows.Forms.ToolStripButton();
            this.autoRefreshLabel = new System.Windows.Forms.ToolStripLabel();
            this.autoRefreshBox = new System.Windows.Forms.ToolStripComboBox();
            this.connectionTabs = new System.Windows.Forms.TabControl();
            this.statusStrip = new System.Windows.Forms.StatusStrip();
            this.statusLabel = new System.Windows.Forms.ToolStripStatusLabel();
            this.statusProgressBar = new System.Windows.Forms.ToolStripProgressBar();
            this.openFileDialog = new System.Windows.Forms.OpenFileDialog();
            this.closeTabButton = new System.Windows.Forms.Button();
            this.autoRefreshTimer = new System.Windows.Forms.Timer(this.components);
            this.toolStrip.SuspendLayout();
            this.statusStrip.SuspendLayout();
            this.SuspendLayout();
            // 
            // toolStrip
            // 
            this.toolStrip.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.newConnectionButton,
            this.openConnectionButton,
            this.toolStripSeparator,
            this.refreshButton,
            this.autoRefreshLabel,
            this.autoRefreshBox});
            this.toolStrip.Location = new System.Drawing.Point(0, 0);
            this.toolStrip.Name = "toolStrip";
            this.toolStrip.Size = new System.Drawing.Size(1019, 25);
            this.toolStrip.TabIndex = 2;
            // 
            // newConnectionButton
            // 
            this.newConnectionButton.DisplayStyle = System.Windows.Forms.ToolStripItemDisplayStyle.Image;
            this.newConnectionButton.Image = ((System.Drawing.Image)(resources.GetObject("newConnectionButton.Image")));
            this.newConnectionButton.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.newConnectionButton.Name = "newConnectionButton";
            this.newConnectionButton.Size = new System.Drawing.Size(23, 22);
            this.newConnectionButton.Text = "New Default Connection";
            this.newConnectionButton.Click += new System.EventHandler(this.newConnectionButton_Click);
            // 
            // openConnectionButton
            // 
            this.openConnectionButton.DisplayStyle = System.Windows.Forms.ToolStripItemDisplayStyle.Image;
            this.openConnectionButton.Image = ((System.Drawing.Image)(resources.GetObject("openConnectionButton.Image")));
            this.openConnectionButton.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.openConnectionButton.Name = "openConnectionButton";
            this.openConnectionButton.Size = new System.Drawing.Size(32, 22);
            this.openConnectionButton.Text = "New Connection From File";
            this.openConnectionButton.ButtonClick += new System.EventHandler(this.openConnectionButton_ButtonClick);
            this.openConnectionButton.DropDownOpening += new System.EventHandler(this.openConnectionButton_DropDownOpening);
            this.openConnectionButton.DropDownItemClicked += new System.Windows.Forms.ToolStripItemClickedEventHandler(this.openConnectionButton_DropDownItemClicked);
            // 
            // toolStripSeparator
            // 
            this.toolStripSeparator.Name = "toolStripSeparator";
            this.toolStripSeparator.Size = new System.Drawing.Size(6, 25);
            this.toolStripSeparator.Visible = false;
            // 
            // refreshButton
            // 
            this.refreshButton.DisplayStyle = System.Windows.Forms.ToolStripItemDisplayStyle.Image;
            this.refreshButton.Image = ((System.Drawing.Image)(resources.GetObject("refreshButton.Image")));
            this.refreshButton.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.refreshButton.Name = "refreshButton";
            this.refreshButton.Size = new System.Drawing.Size(23, 22);
            this.refreshButton.Text = "Refresh Roots";
            this.refreshButton.Visible = false;
            this.refreshButton.Click += new System.EventHandler(this.refreshButton_Click);
            // 
            // autoRefreshLabel
            // 
            this.autoRefreshLabel.Name = "autoRefreshLabel";
            this.autoRefreshLabel.Size = new System.Drawing.Size(78, 22);
            this.autoRefreshLabel.Text = "Auto Refresh:";
            this.autoRefreshLabel.Visible = false;
            // 
            // autoRefreshBox
            // 
            this.autoRefreshBox.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.autoRefreshBox.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.autoRefreshBox.Items.AddRange(new object[] {
            "Never",
            "1 second",
            "5 seconds",
            "30 seconds"});
            this.autoRefreshBox.Name = "autoRefreshBox";
            this.autoRefreshBox.Size = new System.Drawing.Size(121, 25);
            this.autoRefreshBox.Visible = false;
            this.autoRefreshBox.SelectedIndexChanged += new System.EventHandler(this.autoRefreshBox_SelectedIndexChanged);
            // 
            // connectionTabs
            // 
            this.connectionTabs.Dock = System.Windows.Forms.DockStyle.Fill;
            this.connectionTabs.Location = new System.Drawing.Point(0, 25);
            this.connectionTabs.Multiline = true;
            this.connectionTabs.Name = "connectionTabs";
            this.connectionTabs.SelectedIndex = 0;
            this.connectionTabs.Size = new System.Drawing.Size(1019, 421);
            this.connectionTabs.TabIndex = 3;
            // 
            // statusStrip
            // 
            this.statusStrip.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.statusLabel,
            this.statusProgressBar});
            this.statusStrip.Location = new System.Drawing.Point(0, 446);
            this.statusStrip.Name = "statusStrip";
            this.statusStrip.Size = new System.Drawing.Size(1019, 22);
            this.statusStrip.TabIndex = 4;
            // 
            // statusLabel
            // 
            this.statusLabel.Name = "statusLabel";
            this.statusLabel.Size = new System.Drawing.Size(26, 17);
            this.statusLabel.Text = "Idle";
            // 
            // statusProgressBar
            // 
            this.statusProgressBar.MarqueeAnimationSpeed = 20;
            this.statusProgressBar.Name = "statusProgressBar";
            this.statusProgressBar.Size = new System.Drawing.Size(300, 16);
            this.statusProgressBar.Style = System.Windows.Forms.ProgressBarStyle.Marquee;
            this.statusProgressBar.Visible = false;
            // 
            // openFileDialog
            // 
            this.openFileDialog.FileName = "fdb.cluster";
            this.openFileDialog.Filter = "FoundationDB Cluster Files|*.cluster";
            // 
            // closeTabButton
            // 
            this.closeTabButton.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.closeTabButton.FlatAppearance.BorderSize = 0;
            this.closeTabButton.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.closeTabButton.Image = ((System.Drawing.Image)(resources.GetObject("closeTabButton.Image")));
            this.closeTabButton.Location = new System.Drawing.Point(992, 28);
            this.closeTabButton.Name = "closeTabButton";
            this.closeTabButton.Size = new System.Drawing.Size(24, 24);
            this.closeTabButton.TabIndex = 5;
            this.closeTabButton.UseVisualStyleBackColor = false;
            this.closeTabButton.Visible = false;
            this.closeTabButton.Click += new System.EventHandler(this.closeTabButton_Click);
            // 
            // autoRefreshTimer
            // 
            this.autoRefreshTimer.Tick += new System.EventHandler(this.autoRefreshTimer_Tick);
            // 
            // MainForm
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(9F, 20F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(1019, 468);
            this.Controls.Add(this.closeTabButton);
            this.Controls.Add(this.connectionTabs);
            this.Controls.Add(this.toolStrip);
            this.Controls.Add(this.statusStrip);
            this.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.Margin = new System.Windows.Forms.Padding(2);
            this.Name = "MainForm";
            this.Text = "AgentDB Admin";
            this.WindowState = System.Windows.Forms.FormWindowState.Maximized;
            this.toolStrip.ResumeLayout(false);
            this.toolStrip.PerformLayout();
            this.statusStrip.ResumeLayout(false);
            this.statusStrip.PerformLayout();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion
        private System.Windows.Forms.ToolStrip toolStrip;
        private System.Windows.Forms.TabControl connectionTabs;
        private System.Windows.Forms.StatusStrip statusStrip;
        private System.Windows.Forms.ToolStripProgressBar statusProgressBar;
        private System.Windows.Forms.ToolStripStatusLabel statusLabel;
        private System.Windows.Forms.ToolStripButton newConnectionButton;
        private System.Windows.Forms.ToolStripSplitButton openConnectionButton;
        private System.Windows.Forms.OpenFileDialog openFileDialog;
        private System.Windows.Forms.Button closeTabButton;
        private System.Windows.Forms.ToolStripSeparator toolStripSeparator;
        private System.Windows.Forms.ToolStripButton refreshButton;
        private System.Windows.Forms.ToolStripLabel autoRefreshLabel;
        private System.Windows.Forms.ToolStripComboBox autoRefreshBox;
        private System.Windows.Forms.Timer autoRefreshTimer;
    }
}

