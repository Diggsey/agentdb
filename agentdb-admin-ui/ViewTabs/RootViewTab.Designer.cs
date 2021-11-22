
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
            System.Windows.Forms.TreeNode treeNode4 = new System.Windows.Forms.TreeNode("Overview");
            System.Windows.Forms.TreeNode treeNode5 = new System.Windows.Forms.TreeNode("Clients");
            System.Windows.Forms.TreeNode treeNode6 = new System.Windows.Forms.TreeNode("Partitions");
            System.Windows.Forms.TableLayoutPanel rightPaneTableLayout;
            System.Windows.Forms.GroupBox aggregatedGroupBox;
            System.Windows.Forms.Label messagesLabel;
            System.Windows.Forms.Label includedPartitionsLabel;
            System.Windows.Forms.ToolStripSeparator messageCtxMenuSeparator;
            System.Windows.Forms.GroupBox detailsGroupBox;
            System.Windows.Forms.Label detailsTypeLabel;
            System.Windows.Forms.Label detailsNameLabel;
            this.treeView = new System.Windows.Forms.TreeView();
            this.aggregatedTableLayout = new System.Windows.Forms.TableLayoutPanel();
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
            this.viewMessageToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.viewRecipientToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.detailsTableLayout = new System.Windows.Forms.TableLayoutPanel();
            this.actionsLabel = new System.Windows.Forms.Label();
            this.recvPartitionBox = new System.Windows.Forms.TextBox();
            this.recvPartitionLabel = new System.Windows.Forms.Label();
            this.sendPartitionBox = new System.Windows.Forms.TextBox();
            this.sendPartitionLabel = new System.Windows.Forms.Label();
            this.lastActiveBox = new System.Windows.Forms.TextBox();
            this.lastActiveLabel = new System.Windows.Forms.Label();
            this.detailsTypeBox = new System.Windows.Forms.TextBox();
            this.detailsNameBox = new System.Windows.Forms.TextBox();
            this.actionsFlowLayout = new System.Windows.Forms.FlowLayoutPanel();
            this.repartitionButton = new System.Windows.Forms.Button();
            this.listAgentsButton = new System.Windows.Forms.Button();
            this.agentCountLabel = new System.Windows.Forms.Label();
            this.agentCountBox = new System.Windows.Forms.TextBox();
            splitContainer = new System.Windows.Forms.SplitContainer();
            rightPaneTableLayout = new System.Windows.Forms.TableLayoutPanel();
            aggregatedGroupBox = new System.Windows.Forms.GroupBox();
            messagesLabel = new System.Windows.Forms.Label();
            includedPartitionsLabel = new System.Windows.Forms.Label();
            messageCtxMenuSeparator = new System.Windows.Forms.ToolStripSeparator();
            detailsGroupBox = new System.Windows.Forms.GroupBox();
            detailsTypeLabel = new System.Windows.Forms.Label();
            detailsNameLabel = new System.Windows.Forms.Label();
            ((System.ComponentModel.ISupportInitialize)(splitContainer)).BeginInit();
            splitContainer.Panel1.SuspendLayout();
            splitContainer.Panel2.SuspendLayout();
            splitContainer.SuspendLayout();
            rightPaneTableLayout.SuspendLayout();
            aggregatedGroupBox.SuspendLayout();
            this.aggregatedTableLayout.SuspendLayout();
            this.messageContextMenu.SuspendLayout();
            detailsGroupBox.SuspendLayout();
            this.detailsTableLayout.SuspendLayout();
            this.actionsFlowLayout.SuspendLayout();
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
            splitContainer.Panel2.Controls.Add(rightPaneTableLayout);
            splitContainer.Size = new System.Drawing.Size(849, 543);
            splitContainer.SplitterDistance = 251;
            splitContainer.TabIndex = 0;
            // 
            // treeView
            // 
            this.treeView.Dock = System.Windows.Forms.DockStyle.Fill;
            this.treeView.Location = new System.Drawing.Point(0, 0);
            this.treeView.Name = "treeView";
            treeNode4.Name = "OverviewNode";
            treeNode4.Text = "Overview";
            treeNode5.Name = "ClientsNode";
            treeNode5.Text = "Clients";
            treeNode6.Name = "PartitionsNode";
            treeNode6.Text = "Partitions";
            this.treeView.Nodes.AddRange(new System.Windows.Forms.TreeNode[] {
            treeNode4,
            treeNode5,
            treeNode6});
            this.treeView.Size = new System.Drawing.Size(251, 543);
            this.treeView.TabIndex = 0;
            this.treeView.AfterSelect += new System.Windows.Forms.TreeViewEventHandler(this.treeView_AfterSelect);
            // 
            // rightPaneTableLayout
            // 
            rightPaneTableLayout.AutoSize = true;
            rightPaneTableLayout.ColumnCount = 1;
            rightPaneTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
            rightPaneTableLayout.Controls.Add(aggregatedGroupBox, 0, 1);
            rightPaneTableLayout.Controls.Add(detailsGroupBox, 0, 0);
            rightPaneTableLayout.Dock = System.Windows.Forms.DockStyle.Top;
            rightPaneTableLayout.Location = new System.Drawing.Point(0, 0);
            rightPaneTableLayout.Name = "rightPaneTableLayout";
            rightPaneTableLayout.RowCount = 2;
            rightPaneTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            rightPaneTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            rightPaneTableLayout.Size = new System.Drawing.Size(594, 525);
            rightPaneTableLayout.TabIndex = 0;
            // 
            // aggregatedGroupBox
            // 
            aggregatedGroupBox.AutoSize = true;
            aggregatedGroupBox.Controls.Add(this.aggregatedTableLayout);
            aggregatedGroupBox.Dock = System.Windows.Forms.DockStyle.Fill;
            aggregatedGroupBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            aggregatedGroupBox.Location = new System.Drawing.Point(3, 226);
            aggregatedGroupBox.Name = "aggregatedGroupBox";
            aggregatedGroupBox.Size = new System.Drawing.Size(588, 296);
            aggregatedGroupBox.TabIndex = 0;
            aggregatedGroupBox.TabStop = false;
            aggregatedGroupBox.Text = "Aggregated";
            // 
            // aggregatedTableLayout
            // 
            this.aggregatedTableLayout.AutoSize = true;
            this.aggregatedTableLayout.ColumnCount = 2;
            this.aggregatedTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            this.aggregatedTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
            this.aggregatedTableLayout.Controls.Add(messagesLabel, 0, 1);
            this.aggregatedTableLayout.Controls.Add(includedPartitionsLabel, 0, 0);
            this.aggregatedTableLayout.Controls.Add(this.includedPartitionsBox, 1, 0);
            this.aggregatedTableLayout.Controls.Add(this.messagesView, 0, 2);
            this.aggregatedTableLayout.Dock = System.Windows.Forms.DockStyle.Fill;
            this.aggregatedTableLayout.Location = new System.Drawing.Point(3, 22);
            this.aggregatedTableLayout.Name = "aggregatedTableLayout";
            this.aggregatedTableLayout.RowCount = 4;
            this.aggregatedTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.aggregatedTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.aggregatedTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.aggregatedTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.aggregatedTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.aggregatedTableLayout.Size = new System.Drawing.Size(582, 271);
            this.aggregatedTableLayout.TabIndex = 1;
            // 
            // messagesLabel
            // 
            messagesLabel.AutoSize = true;
            this.aggregatedTableLayout.SetColumnSpan(messagesLabel, 2);
            messagesLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            messagesLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            messagesLabel.Location = new System.Drawing.Point(3, 25);
            messagesLabel.Name = "messagesLabel";
            messagesLabel.Size = new System.Drawing.Size(576, 20);
            messagesLabel.TabIndex = 4;
            messagesLabel.Text = "In-flight messages:";
            messagesLabel.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
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
            this.includedPartitionsBox.Size = new System.Drawing.Size(427, 19);
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
            this.aggregatedTableLayout.SetColumnSpan(this.messagesView, 2);
            this.messagesView.ContextMenuStrip = this.messageContextMenu;
            this.messagesView.Dock = System.Windows.Forms.DockStyle.Fill;
            this.messagesView.Font = new System.Drawing.Font("Consolas", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.messagesView.FullRowSelect = true;
            this.messagesView.HideSelection = false;
            this.messagesView.Location = new System.Drawing.Point(3, 48);
            this.messagesView.Name = "messagesView";
            this.messagesView.Size = new System.Drawing.Size(576, 200);
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
            this.messagesScheduledForColumn.Width = 180;
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
            messageCtxMenuSeparator,
            this.viewMessageToolStripMenuItem,
            this.viewRecipientToolStripMenuItem});
            this.messageContextMenu.Name = "messageContextMenu";
            this.messageContextMenu.Size = new System.Drawing.Size(169, 120);
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
            // messageCtxMenuSeparator
            // 
            messageCtxMenuSeparator.Name = "messageCtxMenuSeparator";
            messageCtxMenuSeparator.Size = new System.Drawing.Size(165, 6);
            // 
            // viewMessageToolStripMenuItem
            // 
            this.viewMessageToolStripMenuItem.Font = new System.Drawing.Font("Segoe UI", 9F, System.Drawing.FontStyle.Bold);
            this.viewMessageToolStripMenuItem.Name = "viewMessageToolStripMenuItem";
            this.viewMessageToolStripMenuItem.Size = new System.Drawing.Size(168, 22);
            this.viewMessageToolStripMenuItem.Text = "View Message";
            this.viewMessageToolStripMenuItem.Click += new System.EventHandler(this.viewMessageToolStripMenuItem_Click);
            // 
            // viewRecipientToolStripMenuItem
            // 
            this.viewRecipientToolStripMenuItem.Name = "viewRecipientToolStripMenuItem";
            this.viewRecipientToolStripMenuItem.Size = new System.Drawing.Size(168, 22);
            this.viewRecipientToolStripMenuItem.Text = "View Recipient";
            this.viewRecipientToolStripMenuItem.Click += new System.EventHandler(this.viewRecipientToolStripMenuItem_Click);
            // 
            // detailsGroupBox
            // 
            detailsGroupBox.AutoSize = true;
            detailsGroupBox.Controls.Add(this.detailsTableLayout);
            detailsGroupBox.Dock = System.Windows.Forms.DockStyle.Fill;
            detailsGroupBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            detailsGroupBox.Location = new System.Drawing.Point(3, 3);
            detailsGroupBox.Name = "detailsGroupBox";
            detailsGroupBox.Size = new System.Drawing.Size(588, 217);
            detailsGroupBox.TabIndex = 1;
            detailsGroupBox.TabStop = false;
            detailsGroupBox.Text = "Details";
            // 
            // detailsTableLayout
            // 
            this.detailsTableLayout.AutoSize = true;
            this.detailsTableLayout.ColumnCount = 2;
            this.detailsTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            this.detailsTableLayout.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
            this.detailsTableLayout.Controls.Add(this.agentCountBox, 0, 5);
            this.detailsTableLayout.Controls.Add(this.agentCountLabel, 0, 5);
            this.detailsTableLayout.Controls.Add(this.actionsLabel, 0, 6);
            this.detailsTableLayout.Controls.Add(this.recvPartitionBox, 1, 4);
            this.detailsTableLayout.Controls.Add(this.recvPartitionLabel, 0, 4);
            this.detailsTableLayout.Controls.Add(this.sendPartitionBox, 1, 3);
            this.detailsTableLayout.Controls.Add(this.sendPartitionLabel, 0, 3);
            this.detailsTableLayout.Controls.Add(this.lastActiveBox, 1, 2);
            this.detailsTableLayout.Controls.Add(this.lastActiveLabel, 0, 2);
            this.detailsTableLayout.Controls.Add(this.detailsTypeBox, 1, 1);
            this.detailsTableLayout.Controls.Add(detailsTypeLabel, 0, 1);
            this.detailsTableLayout.Controls.Add(detailsNameLabel, 0, 0);
            this.detailsTableLayout.Controls.Add(this.detailsNameBox, 1, 0);
            this.detailsTableLayout.Controls.Add(this.actionsFlowLayout, 1, 6);
            this.detailsTableLayout.Dock = System.Windows.Forms.DockStyle.Fill;
            this.detailsTableLayout.Location = new System.Drawing.Point(3, 22);
            this.detailsTableLayout.Name = "detailsTableLayout";
            this.detailsTableLayout.RowCount = 7;
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.detailsTableLayout.Size = new System.Drawing.Size(582, 192);
            this.detailsTableLayout.TabIndex = 0;
            // 
            // actionsLabel
            // 
            this.actionsLabel.AutoSize = true;
            this.actionsLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.actionsLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.actionsLabel.Location = new System.Drawing.Point(3, 150);
            this.actionsLabel.Name = "actionsLabel";
            this.actionsLabel.Size = new System.Drawing.Size(176, 42);
            this.actionsLabel.TabIndex = 11;
            this.actionsLabel.Text = "Actions:";
            this.actionsLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // recvPartitionBox
            // 
            this.recvPartitionBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.recvPartitionBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.recvPartitionBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.recvPartitionBox.Location = new System.Drawing.Point(185, 103);
            this.recvPartitionBox.Name = "recvPartitionBox";
            this.recvPartitionBox.ReadOnly = true;
            this.recvPartitionBox.Size = new System.Drawing.Size(394, 19);
            this.recvPartitionBox.TabIndex = 9;
            // 
            // recvPartitionLabel
            // 
            this.recvPartitionLabel.AutoSize = true;
            this.recvPartitionLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.recvPartitionLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.recvPartitionLabel.Location = new System.Drawing.Point(3, 100);
            this.recvPartitionLabel.Name = "recvPartitionLabel";
            this.recvPartitionLabel.Size = new System.Drawing.Size(176, 25);
            this.recvPartitionLabel.TabIndex = 8;
            this.recvPartitionLabel.Text = "Receive partition range:";
            this.recvPartitionLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // sendPartitionBox
            // 
            this.sendPartitionBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.sendPartitionBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.sendPartitionBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.sendPartitionBox.Location = new System.Drawing.Point(185, 78);
            this.sendPartitionBox.Name = "sendPartitionBox";
            this.sendPartitionBox.ReadOnly = true;
            this.sendPartitionBox.Size = new System.Drawing.Size(394, 19);
            this.sendPartitionBox.TabIndex = 7;
            // 
            // sendPartitionLabel
            // 
            this.sendPartitionLabel.AutoSize = true;
            this.sendPartitionLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.sendPartitionLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.sendPartitionLabel.Location = new System.Drawing.Point(3, 75);
            this.sendPartitionLabel.Name = "sendPartitionLabel";
            this.sendPartitionLabel.Size = new System.Drawing.Size(176, 25);
            this.sendPartitionLabel.TabIndex = 6;
            this.sendPartitionLabel.Text = "Send partition range:";
            this.sendPartitionLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // lastActiveBox
            // 
            this.lastActiveBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.lastActiveBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lastActiveBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.lastActiveBox.Location = new System.Drawing.Point(185, 53);
            this.lastActiveBox.Name = "lastActiveBox";
            this.lastActiveBox.ReadOnly = true;
            this.lastActiveBox.Size = new System.Drawing.Size(394, 19);
            this.lastActiveBox.TabIndex = 5;
            // 
            // lastActiveLabel
            // 
            this.lastActiveLabel.AutoSize = true;
            this.lastActiveLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.lastActiveLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.lastActiveLabel.Location = new System.Drawing.Point(3, 50);
            this.lastActiveLabel.Name = "lastActiveLabel";
            this.lastActiveLabel.Size = new System.Drawing.Size(176, 25);
            this.lastActiveLabel.TabIndex = 4;
            this.lastActiveLabel.Text = "Last active:";
            this.lastActiveLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // detailsTypeBox
            // 
            this.detailsTypeBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.detailsTypeBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.detailsTypeBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.detailsTypeBox.Location = new System.Drawing.Point(185, 28);
            this.detailsTypeBox.Name = "detailsTypeBox";
            this.detailsTypeBox.ReadOnly = true;
            this.detailsTypeBox.Size = new System.Drawing.Size(394, 19);
            this.detailsTypeBox.TabIndex = 3;
            // 
            // detailsTypeLabel
            // 
            detailsTypeLabel.AutoSize = true;
            detailsTypeLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            detailsTypeLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            detailsTypeLabel.Location = new System.Drawing.Point(3, 25);
            detailsTypeLabel.Name = "detailsTypeLabel";
            detailsTypeLabel.Size = new System.Drawing.Size(176, 25);
            detailsTypeLabel.TabIndex = 2;
            detailsTypeLabel.Text = "Type:";
            detailsTypeLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // detailsNameLabel
            // 
            detailsNameLabel.AutoSize = true;
            detailsNameLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            detailsNameLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            detailsNameLabel.Location = new System.Drawing.Point(3, 0);
            detailsNameLabel.Name = "detailsNameLabel";
            detailsNameLabel.Size = new System.Drawing.Size(176, 25);
            detailsNameLabel.TabIndex = 0;
            detailsNameLabel.Text = "Name:";
            detailsNameLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // detailsNameBox
            // 
            this.detailsNameBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.detailsNameBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.detailsNameBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.detailsNameBox.Location = new System.Drawing.Point(185, 3);
            this.detailsNameBox.Name = "detailsNameBox";
            this.detailsNameBox.ReadOnly = true;
            this.detailsNameBox.Size = new System.Drawing.Size(394, 19);
            this.detailsNameBox.TabIndex = 1;
            // 
            // actionsFlowLayout
            // 
            this.actionsFlowLayout.AutoSize = true;
            this.actionsFlowLayout.Controls.Add(this.repartitionButton);
            this.actionsFlowLayout.Controls.Add(this.listAgentsButton);
            this.actionsFlowLayout.Dock = System.Windows.Forms.DockStyle.Fill;
            this.actionsFlowLayout.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.actionsFlowLayout.Location = new System.Drawing.Point(185, 153);
            this.actionsFlowLayout.Name = "actionsFlowLayout";
            this.actionsFlowLayout.Size = new System.Drawing.Size(394, 36);
            this.actionsFlowLayout.TabIndex = 10;
            // 
            // repartitionButton
            // 
            this.repartitionButton.AutoSize = true;
            this.repartitionButton.Location = new System.Drawing.Point(3, 3);
            this.repartitionButton.Name = "repartitionButton";
            this.repartitionButton.Size = new System.Drawing.Size(114, 30);
            this.repartitionButton.TabIndex = 0;
            this.repartitionButton.Text = "Re-partition";
            this.repartitionButton.UseVisualStyleBackColor = true;
            this.repartitionButton.Click += new System.EventHandler(this.repartitionButton_Click);
            // 
            // listAgentsButton
            // 
            this.listAgentsButton.AutoSize = true;
            this.listAgentsButton.Location = new System.Drawing.Point(123, 3);
            this.listAgentsButton.Name = "listAgentsButton";
            this.listAgentsButton.Size = new System.Drawing.Size(114, 30);
            this.listAgentsButton.TabIndex = 1;
            this.listAgentsButton.Text = "List agents";
            this.listAgentsButton.UseVisualStyleBackColor = true;
            this.listAgentsButton.Click += new System.EventHandler(this.listAgentsButton_Click);
            // 
            // agentCountLabel
            // 
            this.agentCountLabel.AutoSize = true;
            this.agentCountLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.agentCountLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.agentCountLabel.Location = new System.Drawing.Point(3, 125);
            this.agentCountLabel.Name = "agentCountLabel";
            this.agentCountLabel.Size = new System.Drawing.Size(176, 25);
            this.agentCountLabel.TabIndex = 12;
            this.agentCountLabel.Text = "Agent count";
            this.agentCountLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // agentCountBox
            // 
            this.agentCountBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.agentCountBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.agentCountBox.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.agentCountBox.Location = new System.Drawing.Point(185, 128);
            this.agentCountBox.Name = "agentCountBox";
            this.agentCountBox.ReadOnly = true;
            this.agentCountBox.Size = new System.Drawing.Size(394, 19);
            this.agentCountBox.TabIndex = 13;
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
            rightPaneTableLayout.ResumeLayout(false);
            rightPaneTableLayout.PerformLayout();
            aggregatedGroupBox.ResumeLayout(false);
            aggregatedGroupBox.PerformLayout();
            this.aggregatedTableLayout.ResumeLayout(false);
            this.aggregatedTableLayout.PerformLayout();
            this.messageContextMenu.ResumeLayout(false);
            detailsGroupBox.ResumeLayout(false);
            detailsGroupBox.PerformLayout();
            this.detailsTableLayout.ResumeLayout(false);
            this.detailsTableLayout.PerformLayout();
            this.actionsFlowLayout.ResumeLayout(false);
            this.actionsFlowLayout.PerformLayout();
            this.ResumeLayout(false);

        }

        #endregion
        private System.Windows.Forms.TreeView treeView;
        private System.Windows.Forms.TableLayoutPanel aggregatedTableLayout;
        private System.Windows.Forms.TextBox includedPartitionsBox;
        private System.Windows.Forms.ListView messagesView;
        private System.Windows.Forms.ColumnHeader messagesScheduledForColumn;
        private System.Windows.Forms.ColumnHeader messagesIdColumn;
        private System.Windows.Forms.ColumnHeader messagesRecipientIdColumn;
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
        private System.Windows.Forms.ToolStripMenuItem viewMessageToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem viewRecipientToolStripMenuItem;
        private System.Windows.Forms.Label actionsLabel;
        private System.Windows.Forms.FlowLayoutPanel actionsFlowLayout;
        private System.Windows.Forms.Button repartitionButton;
        private System.Windows.Forms.Button listAgentsButton;
        private System.Windows.Forms.TextBox agentCountBox;
        private System.Windows.Forms.Label agentCountLabel;
    }
}
