
namespace AgentdbAdmin
{
    partial class RootViewTab
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
            this.components = new System.ComponentModel.Container();
            System.Windows.Forms.SplitContainer splitContainer;
            System.Windows.Forms.TreeNode treeNode7 = new System.Windows.Forms.TreeNode("Overview");
            System.Windows.Forms.TreeNode treeNode8 = new System.Windows.Forms.TreeNode("Clients");
            System.Windows.Forms.TreeNode treeNode9 = new System.Windows.Forms.TreeNode("Partitions");
            System.Windows.Forms.Label messagesLabel;
            System.Windows.Forms.Label agentCountLabel;
            System.Windows.Forms.Label includedPartitionsLabel;
            System.Windows.Forms.Label label2;
            System.Windows.Forms.Label label1;
            this.treeView = new System.Windows.Forms.TreeView();
            this.tableLayoutPanel1 = new System.Windows.Forms.TableLayoutPanel();
            this.groupBox1 = new System.Windows.Forms.GroupBox();
            this.tableLayoutPanel3 = new System.Windows.Forms.TableLayoutPanel();
            this.agentCountBox = new System.Windows.Forms.TextBox();
            this.includedPartitionsBox = new System.Windows.Forms.TextBox();
            this.messagesView = new System.Windows.Forms.ListView();
            this.messagesPartitionColumn = ((System.Windows.Forms.ColumnHeader)(new System.Windows.Forms.ColumnHeader()));
            this.messagesBatchedColumn = ((System.Windows.Forms.ColumnHeader)(new System.Windows.Forms.ColumnHeader()));
            this.messagesScheduledForColumn = ((System.Windows.Forms.ColumnHeader)(new System.Windows.Forms.ColumnHeader()));
            this.messagesIdColumn = ((System.Windows.Forms.ColumnHeader)(new System.Windows.Forms.ColumnHeader()));
            this.messagesRecipientIdColumn = ((System.Windows.Forms.ColumnHeader)(new System.Windows.Forms.ColumnHeader()));
            this.messageContextMenu = new System.Windows.Forms.ContextMenuStrip(this.components);
            this.copyDetailsToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.copyMessageIDToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.copyRecipientIDToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.toolStripSeparator1 = new System.Windows.Forms.ToolStripSeparator();
            this.viewMessageToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.viewRecipientToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.groupBox2 = new System.Windows.Forms.GroupBox();
            this.detailsTableLayout = new System.Windows.Forms.TableLayoutPanel();
            this.recvPartitionBox = new System.Windows.Forms.TextBox();
            this.recvPartitionLabel = new System.Windows.Forms.Label();
            this.sendPartitionBox = new System.Windows.Forms.TextBox();
            this.sendPartitionLabel = new System.Windows.Forms.Label();
            this.lastActiveBox = new System.Windows.Forms.TextBox();
            this.lastActiveLabel = new System.Windows.Forms.Label();
            this.detailsTypeBox = new System.Windows.Forms.TextBox();
            this.detailsNameBox = new System.Windows.Forms.TextBox();
            splitContainer = new System.Windows.Forms.SplitContainer();
            messagesLabel = new System.Windows.Forms.Label();
            agentCountLabel = new System.Windows.Forms.Label();
            includedPartitionsLabel = new System.Windows.Forms.Label();
            label2 = new System.Windows.Forms.Label();
            label1 = new System.Windows.Forms.Label();
            ((System.ComponentModel.ISupportInitialize)(splitContainer)).BeginInit();
            splitContainer.Panel1.SuspendLayout();
            splitContainer.Panel2.SuspendLayout();
            splitContainer.SuspendLayout();
            this.tableLayoutPanel1.SuspendLayout();
            this.groupBox1.SuspendLayout();
            this.tableLayoutPanel3.SuspendLayout();
            this.messageContextMenu.SuspendLayout();
            this.groupBox2.SuspendLayout();
            this.detailsTableLayout.SuspendLayout();
            this.SuspendLayout();
            // 
            // splitContainer
            // 
            splitContainer.Dock = System.Windows.Forms.DockStyle.Fill;
            splitContainer.FixedPanel = System.Windows.Forms.FixedPanel.Panel1;
            splitContainer.Location = new System.Drawing.Point(0, 0);
            splitContainer.Name = "splitContainer";
            // 
            // splitContainer.Panel1
            // 
            splitContainer.Panel1.Controls.Add(this.treeView);
            // 
            // splitContainer.Panel2
            // 
            splitContainer.Panel2.AutoScroll = true;
            splitContainer.Panel2.Controls.Add(this.tableLayoutPanel1);
            splitContainer.Size = new System.Drawing.Size(849, 543);
            splitContainer.SplitterDistance = 200;
            splitContainer.TabIndex = 0;
            // 
            // treeView
            // 
            this.treeView.Dock = System.Windows.Forms.DockStyle.Fill;
            this.treeView.Location = new System.Drawing.Point(0, 0);
            this.treeView.Name = "treeView";
            treeNode7.Name = "OverviewNode";
            treeNode7.Text = "Overview";
            treeNode8.Name = "ClientsNode";
            treeNode8.Text = "Clients";
            treeNode9.Name = "PartitionsNode";
            treeNode9.Text = "Partitions";
            this.treeView.Nodes.AddRange(new System.Windows.Forms.TreeNode[] {
            treeNode7,
            treeNode8,
            treeNode9});
            this.treeView.Size = new System.Drawing.Size(200, 543);
            this.treeView.TabIndex = 0;
            this.treeView.AfterSelect += new System.Windows.Forms.TreeViewEventHandler(this.treeView_AfterSelect);
            // 
            // tableLayoutPanel1
            // 
            this.tableLayoutPanel1.AutoSize = true;
            this.tableLayoutPanel1.ColumnCount = 1;
            this.tableLayoutPanel1.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
            this.tableLayoutPanel1.Controls.Add(this.groupBox1, 0, 1);
            this.tableLayoutPanel1.Controls.Add(this.groupBox2, 0, 0);
            this.tableLayoutPanel1.Dock = System.Windows.Forms.DockStyle.Top;
            this.tableLayoutPanel1.Location = new System.Drawing.Point(0, 0);
            this.tableLayoutPanel1.Name = "tableLayoutPanel1";
            this.tableLayoutPanel1.RowCount = 2;
            this.tableLayoutPanel1.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.tableLayoutPanel1.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.tableLayoutPanel1.Size = new System.Drawing.Size(645, 483);
            this.tableLayoutPanel1.TabIndex = 0;
            // 
            // groupBox1
            // 
            this.groupBox1.AutoSize = true;
            this.groupBox1.Controls.Add(this.tableLayoutPanel3);
            this.groupBox1.Dock = System.Windows.Forms.DockStyle.Fill;
            this.groupBox1.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.groupBox1.Location = new System.Drawing.Point(3, 159);
            this.groupBox1.Name = "groupBox1";
            this.groupBox1.Size = new System.Drawing.Size(639, 321);
            this.groupBox1.TabIndex = 0;
            this.groupBox1.TabStop = false;
            this.groupBox1.Text = "Aggregated";
            // 
            // tableLayoutPanel3
            // 
            this.tableLayoutPanel3.AutoSize = true;
            this.tableLayoutPanel3.ColumnCount = 2;
            this.tableLayoutPanel3.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            this.tableLayoutPanel3.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
            this.tableLayoutPanel3.Controls.Add(messagesLabel, 0, 2);
            this.tableLayoutPanel3.Controls.Add(this.agentCountBox, 1, 1);
            this.tableLayoutPanel3.Controls.Add(agentCountLabel, 0, 1);
            this.tableLayoutPanel3.Controls.Add(includedPartitionsLabel, 0, 0);
            this.tableLayoutPanel3.Controls.Add(this.includedPartitionsBox, 1, 0);
            this.tableLayoutPanel3.Controls.Add(this.messagesView, 0, 3);
            this.tableLayoutPanel3.Dock = System.Windows.Forms.DockStyle.Fill;
            this.tableLayoutPanel3.Location = new System.Drawing.Point(3, 22);
            this.tableLayoutPanel3.Name = "tableLayoutPanel3";
            this.tableLayoutPanel3.RowCount = 5;
            this.tableLayoutPanel3.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.tableLayoutPanel3.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.tableLayoutPanel3.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.tableLayoutPanel3.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.tableLayoutPanel3.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tableLayoutPanel3.Size = new System.Drawing.Size(633, 296);
            this.tableLayoutPanel3.TabIndex = 1;
            // 
            // messagesLabel
            // 
            messagesLabel.AutoSize = true;
            this.tableLayoutPanel3.SetColumnSpan(messagesLabel, 2);
            messagesLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            messagesLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            messagesLabel.Location = new System.Drawing.Point(3, 50);
            messagesLabel.Name = "messagesLabel";
            messagesLabel.Size = new System.Drawing.Size(627, 20);
            messagesLabel.TabIndex = 4;
            messagesLabel.Text = "Messages:";
            messagesLabel.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // agentCountBox
            // 
            this.agentCountBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.agentCountBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.agentCountBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.agentCountBox.Location = new System.Drawing.Point(152, 28);
            this.agentCountBox.Name = "agentCountBox";
            this.agentCountBox.ReadOnly = true;
            this.agentCountBox.Size = new System.Drawing.Size(478, 19);
            this.agentCountBox.TabIndex = 3;
            // 
            // agentCountLabel
            // 
            agentCountLabel.AutoSize = true;
            agentCountLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            agentCountLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            agentCountLabel.Location = new System.Drawing.Point(3, 25);
            agentCountLabel.Name = "agentCountLabel";
            agentCountLabel.Size = new System.Drawing.Size(143, 25);
            agentCountLabel.TabIndex = 2;
            agentCountLabel.Text = "Agent count";
            agentCountLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // includedPartitionsLabel
            // 
            includedPartitionsLabel.AutoSize = true;
            includedPartitionsLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            includedPartitionsLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            includedPartitionsLabel.Location = new System.Drawing.Point(3, 0);
            includedPartitionsLabel.Name = "includedPartitionsLabel";
            includedPartitionsLabel.Size = new System.Drawing.Size(143, 25);
            includedPartitionsLabel.TabIndex = 0;
            includedPartitionsLabel.Text = "Included partitions:";
            includedPartitionsLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // includedPartitionsBox
            // 
            this.includedPartitionsBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.includedPartitionsBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.includedPartitionsBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.includedPartitionsBox.Location = new System.Drawing.Point(152, 3);
            this.includedPartitionsBox.Name = "includedPartitionsBox";
            this.includedPartitionsBox.ReadOnly = true;
            this.includedPartitionsBox.Size = new System.Drawing.Size(478, 19);
            this.includedPartitionsBox.TabIndex = 1;
            // 
            // messagesView
            // 
            this.messagesView.Columns.AddRange(new System.Windows.Forms.ColumnHeader[] {
            this.messagesPartitionColumn,
            this.messagesBatchedColumn,
            this.messagesScheduledForColumn,
            this.messagesIdColumn,
            this.messagesRecipientIdColumn});
            this.tableLayoutPanel3.SetColumnSpan(this.messagesView, 2);
            this.messagesView.ContextMenuStrip = this.messageContextMenu;
            this.messagesView.Dock = System.Windows.Forms.DockStyle.Fill;
            this.messagesView.Font = new System.Drawing.Font("Consolas", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.messagesView.FullRowSelect = true;
            this.messagesView.HideSelection = false;
            this.messagesView.Location = new System.Drawing.Point(3, 73);
            this.messagesView.Name = "messagesView";
            this.messagesView.Size = new System.Drawing.Size(627, 200);
            this.messagesView.TabIndex = 5;
            this.messagesView.UseCompatibleStateImageBehavior = false;
            this.messagesView.View = System.Windows.Forms.View.Details;
            this.messagesView.MouseDoubleClick += new System.Windows.Forms.MouseEventHandler(this.messagesView_MouseDoubleClick);
            // 
            // messagesPartitionColumn
            // 
            this.messagesPartitionColumn.Text = "Partition";
            this.messagesPartitionColumn.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
            this.messagesPartitionColumn.Width = 80;
            // 
            // messagesBatchedColumn
            // 
            this.messagesBatchedColumn.Text = "Batched";
            this.messagesBatchedColumn.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
            this.messagesBatchedColumn.Width = 70;
            // 
            // messagesScheduledForColumn
            // 
            this.messagesScheduledForColumn.Text = "Scheduled";
            this.messagesScheduledForColumn.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
            this.messagesScheduledForColumn.Width = 100;
            // 
            // messagesIdColumn
            // 
            this.messagesIdColumn.Text = "Message ID";
            this.messagesIdColumn.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
            this.messagesIdColumn.Width = 280;
            // 
            // messagesRecipientIdColumn
            // 
            this.messagesRecipientIdColumn.Text = "Recipient ID";
            this.messagesRecipientIdColumn.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
            this.messagesRecipientIdColumn.Width = 280;
            // 
            // messageContextMenu
            // 
            this.messageContextMenu.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.copyDetailsToolStripMenuItem,
            this.copyMessageIDToolStripMenuItem,
            this.copyRecipientIDToolStripMenuItem,
            this.toolStripSeparator1,
            this.viewMessageToolStripMenuItem,
            this.viewRecipientToolStripMenuItem});
            this.messageContextMenu.Name = "messageContextMenu";
            this.messageContextMenu.Size = new System.Drawing.Size(181, 142);
            // 
            // copyDetailsToolStripMenuItem
            // 
            this.copyDetailsToolStripMenuItem.Name = "copyDetailsToolStripMenuItem";
            this.copyDetailsToolStripMenuItem.Size = new System.Drawing.Size(168, 22);
            this.copyDetailsToolStripMenuItem.Text = "Copy Details";
            this.copyDetailsToolStripMenuItem.Click += new System.EventHandler(this.copyDetailsToolStripMenuItem_Click);
            // 
            // copyMessageIDToolStripMenuItem
            // 
            this.copyMessageIDToolStripMenuItem.Name = "copyMessageIDToolStripMenuItem";
            this.copyMessageIDToolStripMenuItem.Size = new System.Drawing.Size(168, 22);
            this.copyMessageIDToolStripMenuItem.Text = "Copy Message ID";
            this.copyMessageIDToolStripMenuItem.Click += new System.EventHandler(this.copyMessageIDToolStripMenuItem_Click);
            // 
            // copyRecipientIDToolStripMenuItem
            // 
            this.copyRecipientIDToolStripMenuItem.Name = "copyRecipientIDToolStripMenuItem";
            this.copyRecipientIDToolStripMenuItem.Size = new System.Drawing.Size(168, 22);
            this.copyRecipientIDToolStripMenuItem.Text = "Copy Recipient ID";
            this.copyRecipientIDToolStripMenuItem.Click += new System.EventHandler(this.copyRecipientIDToolStripMenuItem_Click);
            // 
            // toolStripSeparator1
            // 
            this.toolStripSeparator1.Name = "toolStripSeparator1";
            this.toolStripSeparator1.Size = new System.Drawing.Size(165, 6);
            // 
            // viewMessageToolStripMenuItem
            // 
            this.viewMessageToolStripMenuItem.Font = new System.Drawing.Font("Segoe UI", 9F, System.Drawing.FontStyle.Bold);
            this.viewMessageToolStripMenuItem.Name = "viewMessageToolStripMenuItem";
            this.viewMessageToolStripMenuItem.Size = new System.Drawing.Size(180, 22);
            this.viewMessageToolStripMenuItem.Text = "View Message";
            this.viewMessageToolStripMenuItem.Click += new System.EventHandler(this.viewMessageToolStripMenuItem_Click);
            // 
            // viewRecipientToolStripMenuItem
            // 
            this.viewRecipientToolStripMenuItem.Name = "viewRecipientToolStripMenuItem";
            this.viewRecipientToolStripMenuItem.Size = new System.Drawing.Size(180, 22);
            this.viewRecipientToolStripMenuItem.Text = "View Recipient";
            this.viewRecipientToolStripMenuItem.Click += new System.EventHandler(this.viewRecipientToolStripMenuItem_Click);
            // 
            // groupBox2
            // 
            this.groupBox2.AutoSize = true;
            this.groupBox2.Controls.Add(this.detailsTableLayout);
            this.groupBox2.Dock = System.Windows.Forms.DockStyle.Fill;
            this.groupBox2.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.groupBox2.Location = new System.Drawing.Point(3, 3);
            this.groupBox2.Name = "groupBox2";
            this.groupBox2.Size = new System.Drawing.Size(639, 150);
            this.groupBox2.TabIndex = 1;
            this.groupBox2.TabStop = false;
            this.groupBox2.Text = "Details";
            // 
            // detailsTableLayout
            // 
            this.detailsTableLayout.AutoSize = true;
            this.detailsTableLayout.ColumnCount = 2;
            this.detailsTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            this.detailsTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
            this.detailsTableLayout.Controls.Add(this.recvPartitionBox, 1, 4);
            this.detailsTableLayout.Controls.Add(this.recvPartitionLabel, 0, 4);
            this.detailsTableLayout.Controls.Add(this.sendPartitionBox, 1, 3);
            this.detailsTableLayout.Controls.Add(this.sendPartitionLabel, 0, 3);
            this.detailsTableLayout.Controls.Add(this.lastActiveBox, 1, 2);
            this.detailsTableLayout.Controls.Add(this.lastActiveLabel, 0, 2);
            this.detailsTableLayout.Controls.Add(this.detailsTypeBox, 1, 1);
            this.detailsTableLayout.Controls.Add(label2, 0, 1);
            this.detailsTableLayout.Controls.Add(label1, 0, 0);
            this.detailsTableLayout.Controls.Add(this.detailsNameBox, 1, 0);
            this.detailsTableLayout.Dock = System.Windows.Forms.DockStyle.Fill;
            this.detailsTableLayout.Location = new System.Drawing.Point(3, 22);
            this.detailsTableLayout.Name = "detailsTableLayout";
            this.detailsTableLayout.RowCount = 5;
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.Size = new System.Drawing.Size(633, 125);
            this.detailsTableLayout.TabIndex = 0;
            // 
            // recvPartitionBox
            // 
            this.recvPartitionBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.recvPartitionBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.recvPartitionBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.recvPartitionBox.Location = new System.Drawing.Point(193, 103);
            this.recvPartitionBox.Name = "recvPartitionBox";
            this.recvPartitionBox.ReadOnly = true;
            this.recvPartitionBox.Size = new System.Drawing.Size(437, 19);
            this.recvPartitionBox.TabIndex = 9;
            // 
            // recvPartitionLabel
            // 
            this.recvPartitionLabel.AutoSize = true;
            this.recvPartitionLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.recvPartitionLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.recvPartitionLabel.Location = new System.Drawing.Point(3, 100);
            this.recvPartitionLabel.Name = "recvPartitionLabel";
            this.recvPartitionLabel.Size = new System.Drawing.Size(184, 25);
            this.recvPartitionLabel.TabIndex = 8;
            this.recvPartitionLabel.Text = "Receive Partition Range:";
            this.recvPartitionLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // sendPartitionBox
            // 
            this.sendPartitionBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.sendPartitionBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.sendPartitionBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.sendPartitionBox.Location = new System.Drawing.Point(193, 78);
            this.sendPartitionBox.Name = "sendPartitionBox";
            this.sendPartitionBox.ReadOnly = true;
            this.sendPartitionBox.Size = new System.Drawing.Size(437, 19);
            this.sendPartitionBox.TabIndex = 7;
            // 
            // sendPartitionLabel
            // 
            this.sendPartitionLabel.AutoSize = true;
            this.sendPartitionLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.sendPartitionLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.sendPartitionLabel.Location = new System.Drawing.Point(3, 75);
            this.sendPartitionLabel.Name = "sendPartitionLabel";
            this.sendPartitionLabel.Size = new System.Drawing.Size(184, 25);
            this.sendPartitionLabel.TabIndex = 6;
            this.sendPartitionLabel.Text = "Send Partition Range:";
            this.sendPartitionLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // lastActiveBox
            // 
            this.lastActiveBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.lastActiveBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lastActiveBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.lastActiveBox.Location = new System.Drawing.Point(193, 53);
            this.lastActiveBox.Name = "lastActiveBox";
            this.lastActiveBox.ReadOnly = true;
            this.lastActiveBox.Size = new System.Drawing.Size(437, 19);
            this.lastActiveBox.TabIndex = 5;
            // 
            // lastActiveLabel
            // 
            this.lastActiveLabel.AutoSize = true;
            this.lastActiveLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lastActiveLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.lastActiveLabel.Location = new System.Drawing.Point(3, 50);
            this.lastActiveLabel.Name = "lastActiveLabel";
            this.lastActiveLabel.Size = new System.Drawing.Size(184, 25);
            this.lastActiveLabel.TabIndex = 4;
            this.lastActiveLabel.Text = "Last Active:";
            this.lastActiveLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // detailsTypeBox
            // 
            this.detailsTypeBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.detailsTypeBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.detailsTypeBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.detailsTypeBox.Location = new System.Drawing.Point(193, 28);
            this.detailsTypeBox.Name = "detailsTypeBox";
            this.detailsTypeBox.ReadOnly = true;
            this.detailsTypeBox.Size = new System.Drawing.Size(437, 19);
            this.detailsTypeBox.TabIndex = 3;
            // 
            // label2
            // 
            label2.AutoSize = true;
            label2.Dock = System.Windows.Forms.DockStyle.Fill;
            label2.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            label2.Location = new System.Drawing.Point(3, 25);
            label2.Name = "label2";
            label2.Size = new System.Drawing.Size(184, 25);
            label2.TabIndex = 2;
            label2.Text = "Type:";
            label2.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // label1
            // 
            label1.AutoSize = true;
            label1.Dock = System.Windows.Forms.DockStyle.Fill;
            label1.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            label1.Location = new System.Drawing.Point(3, 0);
            label1.Name = "label1";
            label1.Size = new System.Drawing.Size(184, 25);
            label1.TabIndex = 0;
            label1.Text = "Name:";
            label1.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // detailsNameBox
            // 
            this.detailsNameBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.detailsNameBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.detailsNameBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.detailsNameBox.Location = new System.Drawing.Point(193, 3);
            this.detailsNameBox.Name = "detailsNameBox";
            this.detailsNameBox.ReadOnly = true;
            this.detailsNameBox.Size = new System.Drawing.Size(437, 19);
            this.detailsNameBox.TabIndex = 1;
            // 
            // RootViewTab
            // 
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.None;
            this.Controls.Add(splitContainer);
            this.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.Name = "RootViewTab";
            this.Size = new System.Drawing.Size(849, 543);
            splitContainer.Panel1.ResumeLayout(false);
            splitContainer.Panel2.ResumeLayout(false);
            splitContainer.Panel2.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(splitContainer)).EndInit();
            splitContainer.ResumeLayout(false);
            this.tableLayoutPanel1.ResumeLayout(false);
            this.tableLayoutPanel1.PerformLayout();
            this.groupBox1.ResumeLayout(false);
            this.groupBox1.PerformLayout();
            this.tableLayoutPanel3.ResumeLayout(false);
            this.tableLayoutPanel3.PerformLayout();
            this.messageContextMenu.ResumeLayout(false);
            this.groupBox2.ResumeLayout(false);
            this.groupBox2.PerformLayout();
            this.detailsTableLayout.ResumeLayout(false);
            this.detailsTableLayout.PerformLayout();
            this.ResumeLayout(false);

        }

        #endregion
        private System.Windows.Forms.TreeView treeView;
        private System.Windows.Forms.TableLayoutPanel tableLayoutPanel1;
        private System.Windows.Forms.GroupBox groupBox1;
        private System.Windows.Forms.TableLayoutPanel tableLayoutPanel3;
        private System.Windows.Forms.TextBox agentCountBox;
        private System.Windows.Forms.TextBox includedPartitionsBox;
        private System.Windows.Forms.ListView messagesView;
        private System.Windows.Forms.ColumnHeader messagesScheduledForColumn;
        private System.Windows.Forms.ColumnHeader messagesIdColumn;
        private System.Windows.Forms.ColumnHeader messagesRecipientIdColumn;
        private System.Windows.Forms.GroupBox groupBox2;
        private System.Windows.Forms.TableLayoutPanel detailsTableLayout;
        private System.Windows.Forms.TextBox recvPartitionBox;
        private System.Windows.Forms.TextBox sendPartitionBox;
        private System.Windows.Forms.TextBox lastActiveBox;
        private System.Windows.Forms.TextBox detailsTypeBox;
        private System.Windows.Forms.TextBox detailsNameBox;
        private System.Windows.Forms.Label recvPartitionLabel;
        private System.Windows.Forms.Label sendPartitionLabel;
        private System.Windows.Forms.Label lastActiveLabel;
        private System.Windows.Forms.ColumnHeader messagesPartitionColumn;
        private System.Windows.Forms.ColumnHeader messagesBatchedColumn;
        private System.Windows.Forms.ContextMenuStrip messageContextMenu;
        private System.Windows.Forms.ToolStripMenuItem copyDetailsToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem copyMessageIDToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem copyRecipientIDToolStripMenuItem;
        private System.Windows.Forms.ToolStripSeparator toolStripSeparator1;
        private System.Windows.Forms.ToolStripMenuItem viewMessageToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem viewRecipientToolStripMenuItem;
    }
}
