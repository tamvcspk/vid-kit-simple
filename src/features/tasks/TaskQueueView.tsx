import { useEffect } from 'react';
import { Button } from 'primereact/button';
import { Card } from 'primereact/card';
import { useTasksStore } from '../../store';
import { TaskList } from '../../components/tasks';
import './TaskQueueView.scss';

export function TaskQueueView() {
  const {
    tasks,
    queue,
    isLoading,
    startQueue,
    pauseQueue,
    cancelQueue,
    is_queue_paused,
    refreshTasks,
    getPendingTasks,
    getRunningTasks,
    getCompletedTasks,
    getFailedTasks
  } = useTasksStore();

  // Refresh tasks on mount
  useEffect(() => {
    refreshTasks();
  }, [refreshTasks]);

  // Get task counts
  const pendingCount = getPendingTasks().length;
  const runningCount = getRunningTasks().length;
  const completedCount = getCompletedTasks().length;
  const failedCount = getFailedTasks().length;
  const totalCount = tasks.length;

  return (
    <div className="task-queue-view">
      <div className="task-queue-header">
        <h1>Task Queue</h1>
        <div className="task-queue-actions">
          <Button
            label="Start Queue"
            icon="pi pi-play"
            severity="success"
            disabled={pendingCount === 0 || runningCount > 0}
            onClick={startQueue}
          />
          <Button
            label={is_queue_paused ? "Resume Queue" : "Pause Queue"}
            icon={is_queue_paused ? "pi pi-play" : "pi pi-pause"}
            severity="warning"
            disabled={runningCount === 0}
            onClick={pauseQueue}
          />
          <Button
            label="Cancel Queue"
            icon="pi pi-times"
            severity="danger"
            disabled={pendingCount === 0 && runningCount === 0}
            onClick={cancelQueue}
          />
        </div>
      </div>

      <div className="task-queue-stats">
        <Card title="Pending" className="task-stat-card">
          <div className="task-stat-value">{pendingCount}</div>
        </Card>
        <Card title="Running" className="task-stat-card">
          <div className="task-stat-value">{runningCount}</div>
        </Card>
        <Card title="Completed" className="task-stat-card">
          <div className="task-stat-value">{completedCount}</div>
        </Card>
        <Card title="Failed" className="task-stat-card">
          <div className="task-stat-value">{failedCount}</div>
        </Card>
        <Card title="Total" className="task-stat-card">
          <div className="task-stat-value">{totalCount}</div>
        </Card>
      </div>

      <div className="task-list-wrapper">
        <TaskList />
      </div>
    </div>
  );
}
