import { useState, useEffect } from 'react';
import { DataTable } from 'primereact/datatable';
import { Column } from 'primereact/column';
import { Button } from 'primereact/button';
import { ProgressBar } from 'primereact/progressbar';
import { Tag } from 'primereact/tag';
import { useTasksStore } from '../../store';
import { Task, TaskStatus } from '../../types';
import './TaskList.scss';

export function TaskList() {
  const {
    tasks,
    queue,
    tasksLoading,
    startTask,
    pauseTask,
    resumeTask,
    cancelTask,
    retryTask,
    removeTask,
    clearCompletedTasks,
    refreshTasks
  } = useTasksStore();

  const [selectedTasks, setSelectedTasks] = useState<Task[]>([]);
  const [expandedRows, setExpandedRows] = useState<any>({});

  // Refresh tasks on mount
  useEffect(() => {
    refreshTasks();
  }, [refreshTasks]);

  // Status template
  const statusTemplate = (rowData: Task) => {
    let severity = '';
    let value = '';

    switch (rowData.status) {
      case TaskStatus.Pending:
        severity = 'info';
        value = 'Pending';
        break;
      case TaskStatus.Running:
        severity = 'warning';
        value = 'Running';
        break;
      case TaskStatus.Paused:
        severity = 'warning';
        value = 'Paused';
        break;
      case TaskStatus.Completed:
        severity = 'success';
        value = 'Completed';
        break;
      case TaskStatus.Failed:
        severity = 'danger';
        value = 'Failed';
        break;
      case TaskStatus.Canceled:
        severity = 'secondary';
        value = 'Canceled';
        break;
      default:
        severity = 'secondary';
        value = 'Unknown';
    }

    return <Tag severity={severity} value={value} />;
  };

  // Progress template
  const progressTemplate = (rowData: Task) => {
    if (rowData.status === TaskStatus.Running) {
      return <ProgressBar value={rowData.progress} showValue={true} />;
    } else if (rowData.status === TaskStatus.Completed) {
      return <ProgressBar value={100} showValue={true} />;
    } else {
      return <ProgressBar value={rowData.progress} showValue={true} />;
    }
  };

  // Actions template
  const actionsTemplate = (rowData: Task) => {
    return (
      <div className="task-actions">
        {rowData.status === TaskStatus.Pending && (
          <Button
            icon="pi pi-play"
            rounded
            text
            severity="success"
            tooltip="Start"
            onClick={() => startTask(rowData.id)}
          />
        )}
        {rowData.status === TaskStatus.Running && (
          <Button
            icon="pi pi-pause"
            rounded
            text
            severity="warning"
            tooltip="Pause"
            onClick={() => pauseTask(rowData.id)}
          />
        )}
        {rowData.status === TaskStatus.Paused && (
          <Button
            icon="pi pi-play"
            rounded
            text
            severity="success"
            tooltip="Resume"
            onClick={() => resumeTask(rowData.id)}
          />
        )}
        {(rowData.status === TaskStatus.Pending || 
          rowData.status === TaskStatus.Running || 
          rowData.status === TaskStatus.Paused) && (
          <Button
            icon="pi pi-times"
            rounded
            text
            severity="danger"
            tooltip="Cancel"
            onClick={() => cancelTask(rowData.id)}
          />
        )}
        {(rowData.status === TaskStatus.Failed || 
          rowData.status === TaskStatus.Canceled) && (
          <Button
            icon="pi pi-refresh"
            rounded
            text
            severity="info"
            tooltip="Retry"
            onClick={() => retryTask(rowData.id)}
          />
        )}
        <Button
          icon="pi pi-trash"
          rounded
          text
          severity="danger"
          tooltip="Remove"
          onClick={() => removeTask(rowData.id)}
        />
      </div>
    );
  };

  // Row expansion template
  const rowExpansionTemplate = (data: Task) => {
    return (
      <div className="task-details">
        <div className="task-detail-row">
          <span className="task-detail-label">Input File:</span>
          <span className="task-detail-value">{data.input_path}</span>
        </div>
        <div className="task-detail-row">
          <span className="task-detail-label">Output File:</span>
          <span className="task-detail-value">{data.output_path}</span>
        </div>
        {data.error && (
          <div className="task-detail-row">
            <span className="task-detail-label">Error:</span>
            <span className="task-detail-value error">{data.error}</span>
          </div>
        )}
        <div className="task-detail-row">
          <span className="task-detail-label">Created:</span>
          <span className="task-detail-value">
            {new Date(data.created_at).toLocaleString()}
          </span>
        </div>
        {data.completed_at && (
          <div className="task-detail-row">
            <span className="task-detail-label">Completed:</span>
            <span className="task-detail-value">
              {new Date(data.completed_at).toLocaleString()}
            </span>
          </div>
        )}
        <div className="task-detail-row">
          <span className="task-detail-label">Attempts:</span>
          <span className="task-detail-value">{data.attempts}</span>
        </div>
      </div>
    );
  };

  // Get task type label
  const getTaskTypeLabel = (type: string) => {
    switch (type) {
      case 'convert':
        return 'Convert';
      case 'split':
        return 'Split';
      case 'edit':
        return 'Edit';
      case 'sanitize':
        return 'Sanitize';
      default:
        return type;
    }
  };

  return (
    <div className="task-list-container">
      <div className="task-list-header">
        <h2>Tasks</h2>
        <div className="task-list-actions">
          <Button
            label="Clear Completed"
            icon="pi pi-trash"
            severity="secondary"
            onClick={clearCompletedTasks}
          />
          <Button
            icon="pi pi-refresh"
            rounded
            severity="info"
            onClick={refreshTasks}
          />
        </div>
      </div>
      
      <DataTable
        value={tasks}
        expandedRows={expandedRows}
        onRowToggle={(e) => setExpandedRows(e.data)}
        rowExpansionTemplate={rowExpansionTemplate}
        selection={selectedTasks}
        onSelectionChange={(e) => setSelectedTasks(e.value as Task[])}
        dataKey="id"
        paginator
        rows={10}
        rowsPerPageOptions={[5, 10, 25, 50]}
        loading={tasksLoading}
        emptyMessage="No tasks found"
        className="task-table"
      >
        <Column expander style={{ width: '3em' }} />
        <Column selectionMode="multiple" style={{ width: '3em' }} />
        <Column field="id" header="ID" style={{ width: '5em' }} />
        <Column
          field="task_type"
          header="Type"
          body={(rowData) => getTaskTypeLabel(rowData.task_type)}
          style={{ width: '8em' }}
        />
        <Column
          field="status"
          header="Status"
          body={statusTemplate}
          style={{ width: '10em' }}
        />
        <Column
          field="progress"
          header="Progress"
          body={progressTemplate}
          style={{ width: '15em' }}
        />
        <Column
          body={actionsTemplate}
          header="Actions"
          style={{ width: '12em', textAlign: 'center' }}
        />
      </DataTable>
    </div>
  );
}
