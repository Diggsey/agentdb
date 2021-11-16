
namespace AgentdbAdmin
{
    partial class AgentListViewTab
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
            System.Windows.Forms.Label agentsPerPageLabel;
            this.agentListBox = new System.Windows.Forms.ListBox();
            this.goToAgentButton = new System.Windows.Forms.Button();
            this.prevPageButton = new System.Windows.Forms.Button();
            this.nextPageButton = new System.Windows.Forms.Button();
            this.agentsPerPageBox = new System.Windows.Forms.NumericUpDown();
            actionsLayoutPanel = new System.Windows.Forms.TableLayoutPanel();
            optionsLayoutPanel = new System.Windows.Forms.FlowLayoutPanel();
            agentsPerPageLabel = new System.Windows.Forms.Label();
            actionsLayoutPanel.SuspendLayout();
            optionsLayoutPanel.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.agentsPerPageBox)).BeginInit();
            this.SuspendLayout();
            // 
            // agentListBox
            // 
            this.agentListBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.agentListBox.Font = new System.Drawing.Font("Consolas", 9.75F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.agentListBox.FormattingEnabled = true;
            this.agentListBox.IntegralHeight = false;
            this.agentListBox.ItemHeight = 15;
            this.agentListBox.Location = new System.Drawing.Point(0, 32);
            this.agentListBox.Name = "agentListBox";
            this.agentListBox.Size = new System.Drawing.Size(815, 421);
            this.agentListBox.TabIndex = 0;
            this.agentListBox.KeyDown += new System.Windows.Forms.KeyEventHandler(this.agentListBox_KeyDown);
            this.agentListBox.MouseDoubleClick += new System.Windows.Forms.MouseEventHandler(this.agentListBox_MouseDoubleClick);
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
            this.goToAgentButton.Text = "Go To Agent";
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
            optionsLayoutPanel.Controls.Add(agentsPerPageLabel);
            optionsLayoutPanel.Controls.Add(this.agentsPerPageBox);
            optionsLayoutPanel.Dock = System.Windows.Forms.DockStyle.Top;
            optionsLayoutPanel.Location = new System.Drawing.Point(0, 0);
            optionsLayoutPanel.Name = "optionsLayoutPanel";
            optionsLayoutPanel.Size = new System.Drawing.Size(815, 32);
            optionsLayoutPanel.TabIndex = 2;
            // 
            // agentsPerPageLabel
            // 
            agentsPerPageLabel.AutoSize = true;
            agentsPerPageLabel.Dock = System.Windows.Forms.DockStyle.Fill;
            agentsPerPageLabel.Location = new System.Drawing.Point(3, 0);
            agentsPerPageLabel.Name = "agentsPerPageLabel";
            agentsPerPageLabel.Size = new System.Drawing.Size(131, 32);
            agentsPerPageLabel.TabIndex = 0;
            agentsPerPageLabel.Text = "Agents per page:";
            agentsPerPageLabel.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // agentsPerPageBox
            // 
            this.agentsPerPageBox.Location = new System.Drawing.Point(140, 3);
            this.agentsPerPageBox.Maximum = new decimal(new int[] {
            10000,
            0,
            0,
            0});
            this.agentsPerPageBox.Minimum = new decimal(new int[] {
            1,
            0,
            0,
            0});
            this.agentsPerPageBox.Name = "agentsPerPageBox";
            this.agentsPerPageBox.Size = new System.Drawing.Size(120, 26);
            this.agentsPerPageBox.TabIndex = 1;
            this.agentsPerPageBox.Value = new decimal(new int[] {
            100,
            0,
            0,
            0});
            this.agentsPerPageBox.ValueChanged += new System.EventHandler(this.agentsPerPageBox_ValueChanged);
            // 
            // AgentListViewTab
            // 
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.None;
            this.Controls.Add(this.agentListBox);
            this.Controls.Add(optionsLayoutPanel);
            this.Controls.Add(actionsLayoutPanel);
            this.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(254)));
            this.Margin = new System.Windows.Forms.Padding(2);
            this.Name = "AgentListViewTab";
            this.Size = new System.Drawing.Size(815, 489);
            actionsLayoutPanel.ResumeLayout(false);
            actionsLayoutPanel.PerformLayout();
            optionsLayoutPanel.ResumeLayout(false);
            optionsLayoutPanel.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.agentsPerPageBox)).EndInit();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.ListBox agentListBox;
        private System.Windows.Forms.Button goToAgentButton;
        private System.Windows.Forms.Button prevPageButton;
        private System.Windows.Forms.Button nextPageButton;
        private System.Windows.Forms.NumericUpDown agentsPerPageBox;
    }
}
