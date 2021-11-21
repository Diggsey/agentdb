
namespace AgentdbAdmin
{
    partial class ConnectionTab
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
            System.Windows.Forms.TableLayoutPanel tableLayoutPanel;
            System.Windows.Forms.GroupBox rootListGroupBox;
            System.Windows.Forms.GroupBox databaseToolsGroupBox;
            System.Windows.Forms.FlowLayoutPanel databaseToolsLayoutPanel;
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(ConnectionTab));
            System.Windows.Forms.GroupBox fdbDirectoryGroupBox;
            this.rootList = new System.Windows.Forms.ListBox();
            this.fdbConsoleButton = new System.Windows.Forms.Button();
            this.tabControl = new System.Windows.Forms.TabControl();
            this.homePage = new System.Windows.Forms.TabPage();
            this.closeTabButton = new System.Windows.Forms.Button();
            this.fdbDirectoryView = new System.Windows.Forms.TreeView();
            tableLayoutPanel = new System.Windows.Forms.TableLayoutPanel();
            rootListGroupBox = new System.Windows.Forms.GroupBox();
            databaseToolsGroupBox = new System.Windows.Forms.GroupBox();
            databaseToolsLayoutPanel = new System.Windows.Forms.FlowLayoutPanel();
            fdbDirectoryGroupBox = new System.Windows.Forms.GroupBox();
            tableLayoutPanel.SuspendLayout();
            rootListGroupBox.SuspendLayout();
            databaseToolsGroupBox.SuspendLayout();
            databaseToolsLayoutPanel.SuspendLayout();
            this.tabControl.SuspendLayout();
            this.homePage.SuspendLayout();
            fdbDirectoryGroupBox.SuspendLayout();
            this.SuspendLayout();
            // 
            // tableLayoutPanel
            // 
            tableLayoutPanel.ColumnCount = 2;
            tableLayoutPanel.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
            tableLayoutPanel.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
            tableLayoutPanel.Controls.Add(fdbDirectoryGroupBox, 0, 1);
            tableLayoutPanel.Controls.Add(rootListGroupBox, 0, 0);
            tableLayoutPanel.Controls.Add(databaseToolsGroupBox, 1, 0);
            tableLayoutPanel.Dock = System.Windows.Forms.DockStyle.Fill;
            tableLayoutPanel.Location = new System.Drawing.Point(2, 2);
            tableLayoutPanel.Name = "tableLayoutPanel";
            tableLayoutPanel.RowCount = 2;
            tableLayoutPanel.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 25F));
            tableLayoutPanel.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 75F));
            tableLayoutPanel.Size = new System.Drawing.Size(676, 410);
            tableLayoutPanel.TabIndex = 1;
            // 
            // rootListGroupBox
            // 
            rootListGroupBox.Controls.Add(this.rootList);
            rootListGroupBox.Dock = System.Windows.Forms.DockStyle.Fill;
            rootListGroupBox.Location = new System.Drawing.Point(3, 3);
            rootListGroupBox.Name = "rootListGroupBox";
            rootListGroupBox.Size = new System.Drawing.Size(332, 96);
            rootListGroupBox.TabIndex = 0;
            rootListGroupBox.TabStop = false;
            rootListGroupBox.Text = "Detected AgentDB Roots";
            // 
            // rootList
            // 
            this.rootList.DisplayMember = "Title";
            this.rootList.Dock = System.Windows.Forms.DockStyle.Fill;
            this.rootList.FormattingEnabled = true;
            this.rootList.ItemHeight = 20;
            this.rootList.Location = new System.Drawing.Point(3, 22);
            this.rootList.Name = "rootList";
            this.rootList.Size = new System.Drawing.Size(326, 71);
            this.rootList.TabIndex = 0;
            this.rootList.ValueMember = "Root";
            this.rootList.KeyDown += new System.Windows.Forms.KeyEventHandler(this.rootList_KeyDown);
            this.rootList.MouseDoubleClick += new System.Windows.Forms.MouseEventHandler(this.rootList_MouseDoubleClick);
            // 
            // databaseToolsGroupBox
            // 
            databaseToolsGroupBox.Controls.Add(databaseToolsLayoutPanel);
            databaseToolsGroupBox.Dock = System.Windows.Forms.DockStyle.Fill;
            databaseToolsGroupBox.Location = new System.Drawing.Point(341, 3);
            databaseToolsGroupBox.Name = "databaseToolsGroupBox";
            tableLayoutPanel.SetRowSpan(databaseToolsGroupBox, 2);
            databaseToolsGroupBox.Size = new System.Drawing.Size(332, 404);
            databaseToolsGroupBox.TabIndex = 1;
            databaseToolsGroupBox.TabStop = false;
            databaseToolsGroupBox.Text = "Database Tools";
            // 
            // databaseToolsLayoutPanel
            // 
            databaseToolsLayoutPanel.Controls.Add(this.fdbConsoleButton);
            databaseToolsLayoutPanel.Dock = System.Windows.Forms.DockStyle.Fill;
            databaseToolsLayoutPanel.Location = new System.Drawing.Point(3, 22);
            databaseToolsLayoutPanel.Name = "databaseToolsLayoutPanel";
            databaseToolsLayoutPanel.Size = new System.Drawing.Size(326, 379);
            databaseToolsLayoutPanel.TabIndex = 1;
            // 
            // fdbConsoleButton
            // 
            this.fdbConsoleButton.Location = new System.Drawing.Point(3, 3);
            this.fdbConsoleButton.Name = "fdbConsoleButton";
            this.fdbConsoleButton.Size = new System.Drawing.Size(146, 66);
            this.fdbConsoleButton.TabIndex = 0;
            this.fdbConsoleButton.Text = "FoundationDB Console";
            this.fdbConsoleButton.UseVisualStyleBackColor = true;
            // 
            // tabControl
            // 
            this.tabControl.Controls.Add(this.homePage);
            this.tabControl.Dock = System.Windows.Forms.DockStyle.Fill;
            this.tabControl.Location = new System.Drawing.Point(0, 0);
            this.tabControl.Margin = new System.Windows.Forms.Padding(2);
            this.tabControl.Name = "tabControl";
            this.tabControl.SelectedIndex = 0;
            this.tabControl.Size = new System.Drawing.Size(688, 447);
            this.tabControl.TabIndex = 0;
            this.tabControl.SelectedIndexChanged += new System.EventHandler(this.tabControl_SelectedIndexChanged);
            this.tabControl.Deselecting += new System.Windows.Forms.TabControlCancelEventHandler(this.tabControl_Deselecting);
            // 
            // homePage
            // 
            this.homePage.Controls.Add(tableLayoutPanel);
            this.homePage.Location = new System.Drawing.Point(4, 29);
            this.homePage.Margin = new System.Windows.Forms.Padding(2);
            this.homePage.Name = "homePage";
            this.homePage.Padding = new System.Windows.Forms.Padding(2);
            this.homePage.Size = new System.Drawing.Size(680, 414);
            this.homePage.TabIndex = 0;
            this.homePage.Text = "Home";
            this.homePage.UseVisualStyleBackColor = true;
            // 
            // closeTabButton
            // 
            this.closeTabButton.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.closeTabButton.FlatAppearance.BorderSize = 0;
            this.closeTabButton.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.closeTabButton.Image = ((System.Drawing.Image)(resources.GetObject("closeTabButton.Image")));
            this.closeTabButton.Location = new System.Drawing.Point(661, 3);
            this.closeTabButton.Name = "closeTabButton";
            this.closeTabButton.Size = new System.Drawing.Size(24, 24);
            this.closeTabButton.TabIndex = 6;
            this.closeTabButton.UseVisualStyleBackColor = false;
            this.closeTabButton.Visible = false;
            this.closeTabButton.Click += new System.EventHandler(this.closeTabButton_Click);
            // 
            // fdbDirectoryGroupBox
            // 
            fdbDirectoryGroupBox.Controls.Add(this.fdbDirectoryView);
            fdbDirectoryGroupBox.Dock = System.Windows.Forms.DockStyle.Fill;
            fdbDirectoryGroupBox.Location = new System.Drawing.Point(3, 105);
            fdbDirectoryGroupBox.Name = "fdbDirectoryGroupBox";
            fdbDirectoryGroupBox.Size = new System.Drawing.Size(332, 302);
            fdbDirectoryGroupBox.TabIndex = 2;
            fdbDirectoryGroupBox.TabStop = false;
            fdbDirectoryGroupBox.Text = "FoundationDB Directory";
            // 
            // fdbDirectoryView
            // 
            this.fdbDirectoryView.Dock = System.Windows.Forms.DockStyle.Fill;
            this.fdbDirectoryView.Location = new System.Drawing.Point(3, 22);
            this.fdbDirectoryView.Name = "fdbDirectoryView";
            this.fdbDirectoryView.Size = new System.Drawing.Size(326, 277);
            this.fdbDirectoryView.TabIndex = 0;
            this.fdbDirectoryView.AfterCollapse += new System.Windows.Forms.TreeViewEventHandler(this.fdbDirectoryView_AfterCollapse);
            this.fdbDirectoryView.BeforeExpand += new System.Windows.Forms.TreeViewCancelEventHandler(this.fdbDirectoryView_BeforeExpand);
            // 
            // ConnectionTab
            // 
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.None;
            this.Controls.Add(this.closeTabButton);
            this.Controls.Add(this.tabControl);
            this.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.Margin = new System.Windows.Forms.Padding(2);
            this.Name = "ConnectionTab";
            this.Size = new System.Drawing.Size(688, 447);
            this.Load += new System.EventHandler(this.ConnectionTab_Load);
            tableLayoutPanel.ResumeLayout(false);
            rootListGroupBox.ResumeLayout(false);
            databaseToolsGroupBox.ResumeLayout(false);
            databaseToolsLayoutPanel.ResumeLayout(false);
            this.tabControl.ResumeLayout(false);
            this.homePage.ResumeLayout(false);
            fdbDirectoryGroupBox.ResumeLayout(false);
            this.ResumeLayout(false);

        }

        #endregion

        private System.Windows.Forms.TabControl tabControl;
        private System.Windows.Forms.TabPage homePage;
        private System.Windows.Forms.ListBox rootList;
        private System.Windows.Forms.Button fdbConsoleButton;
        private System.Windows.Forms.Button closeTabButton;
        private System.Windows.Forms.TreeView fdbDirectoryView;
    }
}
