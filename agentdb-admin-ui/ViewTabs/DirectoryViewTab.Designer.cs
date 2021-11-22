
namespace AgentdbAdmin
{
    partial class DirectoryViewTab
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
            System.Windows.Forms.TableLayoutPanel actionsLayoutPanel;
            System.Windows.Forms.FlowLayoutPanel optionsLayoutPanel;
            System.Windows.Forms.Label itemsPerPageLabel;
            System.Windows.Forms.Label directoryPathLabel;
            System.Windows.Forms.Label layerLabel;
            this.goToAgentButton = new System.Windows.Forms.Button();
            this.prevPageButton = new System.Windows.Forms.Button();
            this.nextPageButton = new System.Windows.Forms.Button();
            this.itemsPerPageBox = new System.Windows.Forms.NumericUpDown();
            this.itemsListView = new System.Windows.Forms.ListView();
            this.directoryPathBox = new System.Windows.Forms.TextBox();
            this.layerBox = new System.Windows.Forms.TextBox();
            actionsLayoutPanel = new System.Windows.Forms.TableLayoutPanel();
            optionsLayoutPanel = new System.Windows.Forms.FlowLayoutPanel();
            itemsPerPageLabel = new System.Windows.Forms.Label();
            directoryPathLabel = new System.Windows.Forms.Label();
            layerLabel = new System.Windows.Forms.Label();
            actionsLayoutPanel.SuspendLayout();
            optionsLayoutPanel.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.itemsPerPageBox)).BeginInit();
            this.SuspendLayout();
            // 
            // actionsLayoutPanel
            // 
            actionsLayoutPanel.AutoSize = true;
            actionsLayoutPanel.ColumnCount = 5;
            actionsLayoutPanel.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            actionsLayoutPanel.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
            actionsLayoutPanel.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            actionsLayoutPanel.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 50F));
            actionsLayoutPanel.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            actionsLayoutPanel.Controls.Add(this.goToAgentButton, 2, 0);
            actionsLayoutPanel.Controls.Add(this.prevPageButton, 0, 0);
            actionsLayoutPanel.Controls.Add(this.nextPageButton, 4, 0);
            actionsLayoutPanel.Dock = System.Windows.Forms.DockStyle.Bottom;
            actionsLayoutPanel.Location = new System.Drawing.Point(0, 453);
            actionsLayoutPanel.Name = "actionsLayoutPanel";
            actionsLayoutPanel.RowCount = 1;
            actionsLayoutPanel.RowStyles.Add(new System.Windows.Forms.RowStyle());
            actionsLayoutPanel.Size = new System.Drawing.Size(815, 36);
            actionsLayoutPanel.TabIndex = 1;
            // 
            // goToAgentButton
            // 
            this.goToAgentButton.AutoSize = true;
            this.goToAgentButton.Location = new System.Drawing.Point(347, 3);
            this.goToAgentButton.Name = "goToAgentButton";
            this.goToAgentButton.Size = new System.Drawing.Size(120, 30);
            this.goToAgentButton.TabIndex = 2;
            this.goToAgentButton.Text = "Go To Key";
            this.goToAgentButton.UseVisualStyleBackColor = true;
            // 
            // prevPageButton
            // 
            this.prevPageButton.AutoSize = true;
            this.prevPageButton.Location = new System.Drawing.Point(3, 3);
            this.prevPageButton.Name = "prevPageButton";
            this.prevPageButton.Size = new System.Drawing.Size(120, 30);
            this.prevPageButton.TabIndex = 0;
            this.prevPageButton.Text = "Previous Page";
            this.prevPageButton.UseVisualStyleBackColor = true;
            this.prevPageButton.Click += new System.EventHandler(this.prevPageButton_Click);
            // 
            // nextPageButton
            // 
            this.nextPageButton.AutoSize = true;
            this.nextPageButton.Location = new System.Drawing.Point(691, 3);
            this.nextPageButton.Name = "nextPageButton";
            this.nextPageButton.Size = new System.Drawing.Size(120, 30);
            this.nextPageButton.TabIndex = 1;
            this.nextPageButton.Text = "Next Page";
            this.nextPageButton.UseVisualStyleBackColor = true;
            this.nextPageButton.Click += new System.EventHandler(this.nextPageButton_Click);
            // 
            // optionsLayoutPanel
            // 
            optionsLayoutPanel.AutoSize = true;
            optionsLayoutPanel.Controls.Add(directoryPathLabel);
            optionsLayoutPanel.Controls.Add(this.directoryPathBox);
            optionsLayoutPanel.Controls.Add(layerLabel);
            optionsLayoutPanel.Controls.Add(this.layerBox);
            optionsLayoutPanel.Controls.Add(itemsPerPageLabel);
            optionsLayoutPanel.Controls.Add(this.itemsPerPageBox);
            optionsLayoutPanel.Dock = System.Windows.Forms.DockStyle.Top;
            optionsLayoutPanel.Location = new System.Drawing.Point(0, 0);
            optionsLayoutPanel.Name = "optionsLayoutPanel";
            optionsLayoutPanel.Size = new System.Drawing.Size(815, 65);
            optionsLayoutPanel.TabIndex = 2;
            // 
            // itemsPerPageLabel
            // 
            itemsPerPageLabel.AutoSize = true;
            itemsPerPageLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            itemsPerPageLabel.Location = new System.Drawing.Point(3, 33);
            itemsPerPageLabel.Name = "itemsPerPageLabel";
            itemsPerPageLabel.Size = new System.Drawing.Size(120, 32);
            itemsPerPageLabel.TabIndex = 0;
            itemsPerPageLabel.Text = "Items per page:";
            itemsPerPageLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // itemsPerPageBox
            // 
            this.itemsPerPageBox.Location = new System.Drawing.Point(129, 36);
            this.itemsPerPageBox.Maximum = new decimal(new int[] {
            10000,
            0,
            0,
            0});
            this.itemsPerPageBox.Minimum = new decimal(new int[] {
            1,
            0,
            0,
            0});
            this.itemsPerPageBox.Name = "itemsPerPageBox";
            this.itemsPerPageBox.Size = new System.Drawing.Size(120, 26);
            this.itemsPerPageBox.TabIndex = 1;
            this.itemsPerPageBox.Value = new decimal(new int[] {
            100,
            0,
            0,
            0});
            this.itemsPerPageBox.ValueChanged += new System.EventHandler(this.agentsPerPageBox_ValueChanged);
            // 
            // itemsListView
            // 
            this.itemsListView.Dock = System.Windows.Forms.DockStyle.Fill;
            this.itemsListView.HideSelection = false;
            this.itemsListView.Location = new System.Drawing.Point(0, 65);
            this.itemsListView.Name = "itemsListView";
            this.itemsListView.Size = new System.Drawing.Size(815, 388);
            this.itemsListView.TabIndex = 3;
            this.itemsListView.UseCompatibleStateImageBehavior = false;
            this.itemsListView.View = System.Windows.Forms.View.Details;
            // 
            // directoryPathLabel
            // 
            directoryPathLabel.AutoSize = true;
            directoryPathLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            directoryPathLabel.Location = new System.Drawing.Point(3, 0);
            directoryPathLabel.Name = "directoryPathLabel";
            directoryPathLabel.Size = new System.Drawing.Size(112, 33);
            directoryPathLabel.TabIndex = 2;
            directoryPathLabel.Text = "Directory path:";
            directoryPathLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // directoryPathBox
            // 
            this.directoryPathBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.directoryPathBox.Location = new System.Drawing.Point(125, 7);
            this.directoryPathBox.Margin = new System.Windows.Forms.Padding(7);
            this.directoryPathBox.Name = "directoryPathBox";
            this.directoryPathBox.ReadOnly = true;
            this.directoryPathBox.Size = new System.Drawing.Size(295, 19);
            this.directoryPathBox.TabIndex = 3;
            // 
            // layerLabel
            // 
            layerLabel.AutoSize = true;
            layerLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            layerLabel.Location = new System.Drawing.Point(430, 0);
            layerLabel.Name = "layerLabel";
            layerLabel.Size = new System.Drawing.Size(52, 33);
            layerLabel.TabIndex = 4;
            layerLabel.Text = "Layer:";
            layerLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // layerBox
            // 
            this.layerBox.BorderStyle = System.Windows.Forms.BorderStyle.None;
            optionsLayoutPanel.SetFlowBreak(this.layerBox, true);
            this.layerBox.Location = new System.Drawing.Point(492, 7);
            this.layerBox.Margin = new System.Windows.Forms.Padding(7);
            this.layerBox.Name = "layerBox";
            this.layerBox.ReadOnly = true;
            this.layerBox.Size = new System.Drawing.Size(125, 19);
            this.layerBox.TabIndex = 5;
            // 
            // DirectoryViewTab
            // 
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.None;
            this.Controls.Add(this.itemsListView);
            this.Controls.Add(optionsLayoutPanel);
            this.Controls.Add(actionsLayoutPanel);
            this.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.Margin = new System.Windows.Forms.Padding(2);
            this.Name = "DirectoryViewTab";
            this.Size = new System.Drawing.Size(815, 489);
            actionsLayoutPanel.ResumeLayout(false);
            actionsLayoutPanel.PerformLayout();
            optionsLayoutPanel.ResumeLayout(false);
            optionsLayoutPanel.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.itemsPerPageBox)).EndInit();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion
        private System.Windows.Forms.Button goToAgentButton;
        private System.Windows.Forms.Button prevPageButton;
        private System.Windows.Forms.Button nextPageButton;
        private System.Windows.Forms.NumericUpDown itemsPerPageBox;
        private System.Windows.Forms.ListView itemsListView;
        private System.Windows.Forms.TextBox directoryPathBox;
        private System.Windows.Forms.TextBox layerBox;
    }
}
