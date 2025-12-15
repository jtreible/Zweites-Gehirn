# Phase 2: ADHD Core Features - Implementation Plan

**Session Date**: Next Session
**Goal**: Visual organization and ADHD-friendly enhancements
**Estimated Duration**: 3-4 sessions
**Status**: Ready to Start

---

## Pre-Implementation Tasks

### 1. Database Verification ✅
**Priority**: HIGH
**Task**: Check database to verify Phase 1 data integrity

**Steps**:
- Install and configure SQLite viewer tool (DB Browser for SQLite or similar)
- Query the database at: `C:\Users\Operation1\AppData\Roaming\zweites-gehirn\db\zweites_gehirn.db`
- Verify tables exist:
  - `users` (should have default user with id=1)
  - `workspaces` (should have default workspace with id=1)
  - `tasks` (should have your test task data)
  - `schema_version` (should show version 1)
- Check data integrity:
  - Verify completed task was marked with `completed_at` timestamp
  - Verify deleted task is actually deleted (hard delete, not soft)
  - Verify new task has proper foreign keys and timestamps
- Document any issues found

**Deliverables**:
- Database verification report
- Screenshots of table contents
- List of any data inconsistencies

---

## Phase 2 Features Overview

### Feature 1: Kanban Board 🎯
**Complexity**: High
**Dependencies**: None
**Estimated Time**: 1.5-2 sessions

### Feature 2: Subtasks & Progress Indicators 📊
**Complexity**: Medium
**Dependencies**: None
**Estimated Time**: 1 session

### Feature 3: Pomodoro Timer ⏱️
**Complexity**: Medium
**Dependencies**: None
**Estimated Time**: 0.5-1 session

### Feature 4: Daily Task Rollover 🌅
**Complexity**: Medium
**Dependencies**: None
**Estimated Time**: 0.5 session

### Feature 5: Visual Enhancements ✨
**Complexity**: Low
**Dependencies**: Kanban Board, Subtasks
**Estimated Time**: 0.5 session

---

## Detailed Implementation Plan

## Feature 1: Kanban Board

### Overview
A drag-and-drop Kanban board with columns: To Do, In Progress, Done. Tasks can be dragged between columns to update their status.

### Database Changes
**File**: `src-tauri/src/db/schema.rs`

Add new migration (v002):
```sql
-- Add status column values: 'todo', 'in_progress', 'done', 'blocked'
-- Already exists in v001, but verify values are correct

-- Add order_index for column sorting (already exists)

-- Add column_position for Kanban
ALTER TABLE tasks ADD COLUMN column_position INTEGER DEFAULT 0;
```

### Backend Changes

**New Files**:
1. `src-tauri/src/commands/kanban.rs`
   - `get_kanban_tasks(status: String) -> Vec<Task>`
   - `move_task_to_column(task_id: i64, new_status: String, position: i32) -> Result<Task>`
   - `reorder_tasks_in_column(task_ids: Vec<i64>) -> Result<()>`

**Modified Files**:
2. `src-tauri/src/main.rs`
   - Register new kanban commands

3. `src-tauri/src/commands/tasks.rs`
   - Add `update_task_status` function

### Frontend Changes

**New Files**:
1. `src/lib/components/kanban/KanbanBoard.svelte`
   - Main board component with 3-4 columns
   - Handles drag-and-drop state

2. `src/lib/components/kanban/KanbanColumn.svelte`
   - Individual column component
   - Drop zone for tasks
   - Column header with task count

3. `src/lib/components/kanban/KanbanCard.svelte`
   - Draggable task card
   - Shows title, priority, time estimate
   - Visual indicators (energy, difficulty)

4. `src/routes/kanban/+page.svelte`
   - Kanban view route
   - Full-screen board layout

**Modified Files**:
5. `src/lib/api/tasks.ts`
   - Add Kanban-specific API wrappers

6. `src/lib/stores/tasks.ts`
   - Add kanban-specific state
   - Add drag-and-drop handlers

**Dependencies to Install**:
```bash
pnpm add @dnd-kit/core @dnd-kit/sortable @dnd-kit/utilities
```

### Success Criteria
- [ ] Can drag tasks between columns
- [ ] Status updates in database immediately
- [ ] Drag-and-drop feels smooth (60fps)
- [ ] Can render 200+ tasks without lag
- [ ] Tasks maintain position within column

---

## Feature 2: Subtasks & Progress Indicators

### Overview
Tasks can have child tasks (subtasks). Parent tasks show progress (e.g., "3/5 subtasks completed").

### Database Changes
**File**: `src-tauri/src/db/schema.rs`

Migration v002 (continued):
```sql
-- parent_task_id already exists from v001
-- Add index for faster subtask queries
CREATE INDEX IF NOT EXISTS idx_tasks_parent_id ON tasks(parent_task_id);
```

### Backend Changes

**Modified Files**:
1. `src-tauri/src/commands/tasks.rs`
   - `create_subtask(parent_id: i64, title: String) -> Result<Task>`
   - `get_task_with_subtasks(task_id: i64) -> Result<TaskWithSubtasks>`
   - `get_subtask_progress(task_id: i64) -> Result<SubtaskProgress>`

**New Types**:
2. `src-tauri/src/db/models.rs`
   ```rust
   pub struct TaskWithSubtasks {
       pub task: Task,
       pub subtasks: Vec<Task>,
       pub progress: SubtaskProgress,
   }

   pub struct SubtaskProgress {
       pub total: i32,
       pub completed: i32,
       pub percentage: f32,
   }
   ```

### Frontend Changes

**New Files**:
1. `src/lib/components/tasks/SubtaskList.svelte`
   - Shows list of subtasks with checkboxes
   - Add new subtask inline

2. `src/lib/components/tasks/ProgressBar.svelte`
   - Visual progress indicator
   - Shows "3/5" text and filled bar

**Modified Files**:
3. `src/lib/components/tasks/TaskCard.svelte`
   - Show progress indicator if task has subtasks
   - Click to expand/collapse subtasks

4. `src/lib/components/tasks/TaskList.svelte`
   - Support nested task display

### Success Criteria
- [ ] Can create subtasks from parent task
- [ ] Progress shows correctly (e.g., "3/5 completed")
- [ ] Completing all subtasks marks parent as eligible for completion
- [ ] Subtasks can be independently completed
- [ ] Nested tasks render correctly in list and Kanban views

---

## Feature 3: Pomodoro Timer

### Overview
Built-in Pomodoro timer (25 min work, 5 min break). Tracks which task you're working on.

### Database Changes
**File**: `src-tauri/src/db/schema.rs`

Migration v002 (continued):
```sql
-- Pomodoro sessions
CREATE TABLE IF NOT EXISTS pomodoro_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER,
    user_id INTEGER NOT NULL DEFAULT 1,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    duration_minutes INTEGER DEFAULT 25,
    completed BOOLEAN DEFAULT false,
    session_type TEXT DEFAULT 'work', -- 'work' or 'break'

    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE SET NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_pomodoro_task_id ON pomodoro_sessions(task_id);
CREATE INDEX IF NOT EXISTS idx_pomodoro_started_at ON pomodoro_sessions(started_at);
```

### Backend Changes

**New Files**:
1. `src-tauri/src/commands/pomodoro.rs`
   - `start_pomodoro(task_id: Option<i64>, duration_minutes: i32) -> Result<PomodoroSession>`
   - `pause_pomodoro(session_id: i64) -> Result<()>`
   - `complete_pomodoro(session_id: i64) -> Result<PomodoroSession>`
   - `get_active_pomodoro() -> Result<Option<PomodoroSession>>`

2. `src-tauri/src/db/models.rs`
   ```rust
   pub struct PomodoroSession {
       pub id: i64,
       pub task_id: Option<i64>,
       pub started_at: String,
       pub ended_at: Option<String>,
       pub duration_minutes: i32,
       pub completed: bool,
       pub session_type: String,
   }
   ```

### Frontend Changes

**New Files**:
1. `src/lib/components/pomodoro/PomodoroTimer.svelte`
   - Circular timer display
   - Start/Pause/Reset buttons
   - Task selector dropdown

2. `src/lib/components/pomodoro/TimerWidget.svelte`
   - Floating timer widget (always visible)
   - Minimized/expanded states

3. `src/lib/stores/pomodoro.ts`
   - Timer state management
   - Countdown logic
   - Audio notifications

**Modified Files**:
4. `src/routes/+layout.svelte`
   - Add floating timer widget to layout

### Success Criteria
- [ ] Timer counts down from 25:00 to 0:00
- [ ] Audio notification plays when timer ends
- [ ] Can start timer from any task
- [ ] Timer persists across page navigation
- [ ] Timer state saved (can resume after app restart)
- [ ] Break timer automatically starts after work session

---

## Feature 4: Daily Task Rollover

### Overview
Automatically moves incomplete scheduled tasks to the next day at midnight.

### Backend Changes

**New Files**:
1. `src-tauri/src/services/rollover_service.rs`
   ```rust
   pub async fn run_daily_rollover() -> Result<RolloverResult> {
       // Find all tasks scheduled for yesterday that aren't completed
       // Move their scheduled_date to today
       // Return count of rolled over tasks
   }

   pub struct RolloverResult {
       pub tasks_rolled_over: i32,
       pub tasks_archived: i32,
   }
   ```

2. `src-tauri/src/scheduler.rs`
   - Background task scheduler using `tokio::time`
   - Runs rollover at midnight local time

**Modified Files**:
3. `src-tauri/src/main.rs`
   - Initialize scheduler on app startup
   - Spawn background rollover task

### Frontend Changes

**New Files**:
1. `src/lib/components/notifications/RolloverNotification.svelte`
   - Toast notification showing rollover results
   - "X tasks moved to today"

**Modified Files**:
2. `src/lib/stores/tasks.ts`
   - Listen for rollover events
   - Refresh task list after rollover

### Success Criteria
- [ ] Rollover runs automatically at midnight
- [ ] Only moves incomplete tasks
- [ ] Only moves tasks with scheduled_date in the past
- [ ] User sees notification on next app open
- [ ] Can trigger manual rollover from settings

---

## Feature 5: Visual Enhancements

### Overview
ADHD-friendly visual indicators and celebration animations.

### Visual Indicators

**Difficulty Peppers** 🌶️:
- 1 pepper = Easy
- 2 peppers = Medium
- 3 peppers = Hard
- 4 peppers = Very Hard
- 5 peppers = Extreme

**Energy Battery** 🔋:
- Low energy = 🔋 (empty battery)
- Medium energy = 🔋🔋 (half battery)
- High energy = 🔋🔋🔋 (full battery)

### Frontend Changes

**New Files**:
1. `src/lib/components/ui/DifficultyIndicator.svelte`
   - Shows pepper icons based on difficulty_level

2. `src/lib/components/ui/EnergyIndicator.svelte`
   - Shows battery icons based on energy_level

3. `src/lib/components/animations/CompletionCelebration.svelte`
   - Confetti animation on task completion
   - Success sound effect

**Modified Files**:
4. `src/lib/components/tasks/TaskCard.svelte`
   - Add difficulty and energy indicators
   - Trigger celebration on completion

### Dependencies
```bash
pnpm add canvas-confetti
```

### Success Criteria
- [ ] Difficulty shown as peppers (1-5)
- [ ] Energy shown as battery icons
- [ ] Confetti animation plays on task completion
- [ ] Sound effect plays (with mute option)
- [ ] Visual indicators clear and not overwhelming

---

## Database Migration Strategy

### Migration v002 Summary
```sql
-- Add column_position for Kanban sorting
ALTER TABLE tasks ADD COLUMN column_position INTEGER DEFAULT 0;

-- Index for subtasks
CREATE INDEX IF NOT EXISTS idx_tasks_parent_id ON tasks(parent_task_id);

-- Pomodoro sessions table
CREATE TABLE IF NOT EXISTS pomodoro_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER,
    user_id INTEGER NOT NULL DEFAULT 1,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    duration_minutes INTEGER DEFAULT 25,
    completed BOOLEAN DEFAULT false,
    session_type TEXT DEFAULT 'work',

    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE SET NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_pomodoro_task_id ON pomodoro_sessions(task_id);
CREATE INDEX IF NOT EXISTS idx_pomodoro_started_at ON pomodoro_sessions(started_at);

-- Update schema version
INSERT INTO schema_version (version) VALUES (2);
```

**File**: `src-tauri/src/db/schema.rs`
**Function**: Add `migration_v002()` function

---

## Implementation Order (Recommended)

### Session 1: Database & Subtasks
1. Verify existing database (30 min)
2. Create migration v002 (30 min)
3. Implement subtasks backend (1 hour)
4. Implement subtasks frontend (1.5 hours)
5. Test subtask functionality (30 min)

### Session 2: Kanban Board - Backend
1. Install drag-and-drop dependencies (15 min)
2. Implement Kanban backend commands (1.5 hours)
3. Write tests for Kanban logic (1 hour)
4. Create basic Kanban components (1.5 hours)

### Session 3: Kanban Board - Frontend
1. Implement drag-and-drop functionality (2 hours)
2. Style Kanban board (1 hour)
3. Add animations and polish (1 hour)
4. Test with large datasets (30 min)

### Session 4: Pomodoro, Rollover & Polish
1. Implement Pomodoro timer (1.5 hours)
2. Implement daily rollover (1 hour)
3. Add visual enhancements (1 hour)
4. Final testing and bug fixes (1 hour)

---

## Testing Checklist

### Unit Tests
- [ ] Subtask creation and completion
- [ ] Kanban status updates
- [ ] Task reordering within columns
- [ ] Pomodoro timer logic
- [ ] Rollover date calculations

### Integration Tests
- [ ] Drag task from To Do to In Progress
- [ ] Complete all subtasks, mark parent complete
- [ ] Start Pomodoro, navigate pages, timer continues
- [ ] Rollover runs at midnight

### Performance Tests
- [ ] Kanban with 500+ tasks renders smoothly
- [ ] Drag-and-drop at 60fps
- [ ] Subtask expansion doesn't lag

### Manual Testing
- [ ] All animations feel good (not janky)
- [ ] Confetti doesn't cover important UI
- [ ] Timer notification audible but not annoying
- [ ] Rollover notification clear and helpful

---

## Success Metrics (Phase 2 Complete)

### Performance
- Kanban board renders 200 tasks at 60fps ✅
- Drag-and-drop feels instant (< 16ms frame time) ✅
- Rollover runs reliably every night ✅

### Functionality
- All features work as specified ✅
- No data loss during migrations ✅
- App remains stable with heavy use ✅

### User Experience
- Visual indicators reduce cognitive load ✅
- Celebrations feel rewarding, not annoying ✅
- Pomodoro timer helps maintain focus ✅

---

## Potential Challenges & Solutions

### Challenge 1: Drag-and-Drop Performance
**Issue**: Kanban might lag with many tasks
**Solution**:
- Use virtual scrolling for large columns
- Limit rendering to visible tasks only
- Optimize React/Svelte reconciliation

### Challenge 2: Timer Precision
**Issue**: JavaScript timers drift over time
**Solution**:
- Use Web Workers for background timer
- Recalculate time remaining from start time, not by counting intervals
- Use high-resolution timestamps

### Challenge 3: Midnight Rollover Timing
**Issue**: User might not have app open at midnight
**Solution**:
- Run rollover check on app startup (if last run > 24h ago)
- Store last rollover timestamp in preferences
- Show notification of rolled-over tasks on next launch

### Challenge 4: Migration Failures
**Issue**: Database migration might fail on some systems
**Solution**:
- Backup database before migration
- Use transactions for all migration steps
- Add rollback logic
- Test migrations on fresh and existing databases

---

## Files to Create/Modify Summary

### Backend (Rust)

**New Files** (11):
1. `src-tauri/src/commands/kanban.rs`
2. `src-tauri/src/commands/pomodoro.rs`
3. `src-tauri/src/services/rollover_service.rs`
4. `src-tauri/src/scheduler.rs`
5. `src-tauri/src/db/repositories/kanban_repo.rs`
6. `src-tauri/src/db/repositories/pomodoro_repo.rs`

**Modified Files** (4):
1. `src-tauri/src/main.rs`
2. `src-tauri/src/db/schema.rs`
3. `src-tauri/src/db/models.rs`
4. `src-tauri/src/commands/tasks.rs`

### Frontend (Svelte)

**New Files** (15):
1. `src/lib/components/kanban/KanbanBoard.svelte`
2. `src/lib/components/kanban/KanbanColumn.svelte`
3. `src/lib/components/kanban/KanbanCard.svelte`
4. `src/lib/components/tasks/SubtaskList.svelte`
5. `src/lib/components/tasks/ProgressBar.svelte`
6. `src/lib/components/pomodoro/PomodoroTimer.svelte`
7. `src/lib/components/pomodoro/TimerWidget.svelte`
8. `src/lib/components/ui/DifficultyIndicator.svelte`
9. `src/lib/components/ui/EnergyIndicator.svelte`
10. `src/lib/components/animations/CompletionCelebration.svelte`
11. `src/lib/components/notifications/RolloverNotification.svelte`
12. `src/lib/stores/pomodoro.ts`
13. `src/routes/kanban/+page.svelte`

**Modified Files** (4):
1. `src/lib/api/tasks.ts`
2. `src/lib/stores/tasks.ts`
3. `src/lib/components/tasks/TaskCard.svelte`
4. `src/routes/+layout.svelte`

**Total**: 30 files (new + modified)

---

## Dependencies to Install

```bash
# Drag-and-drop library
pnpm add @dnd-kit/core @dnd-kit/sortable @dnd-kit/utilities

# Confetti animations
pnpm add canvas-confetti

# Type definitions
pnpm add -D @types/canvas-confetti
```

---

## Next Session Preparation

### Before Starting:
1. ✅ Review this plan thoroughly
2. ✅ Check database contents
3. ✅ Ensure dev environment is running (`pnpm tauri:dev`)
4. ✅ Create a git branch: `git checkout -b phase-2-adhd-features`
5. ✅ Backup database before migration

### First Task:
Start with database verification, then create migration v002.

### Questions to Resolve:
- Should rollover also archive old completed tasks?
- Should Pomodoro timer show desktop notifications?
- Default Pomodoro duration (25 min or user-configurable)?
- Should tasks in "Done" column auto-archive after X days?

---

## Phase 2 Completion Criteria

Phase 2 is complete when:
- [ ] All 5 features implemented and tested
- [ ] Database migration runs successfully
- [ ] No regressions in Phase 1 features
- [ ] Performance metrics met
- [ ] Code reviewed and documented
- [ ] User can use Kanban board effectively
- [ ] Subtasks help break down large tasks
- [ ] Pomodoro timer aids focus
- [ ] Rollover keeps scheduled tasks current
- [ ] Visual enhancements reduce cognitive load

---

**Ready to begin Phase 2!** 🚀
