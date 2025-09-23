# Task Domain Requirements Specification

## 1. Overview
The Task domain represents individual work items that contribute to project completion. Tasks are the fundamental unit of work tracking in the project management system.

## 2. Functional Requirements

### 2.1 Task Creation (FR-T-001)
**Description**: Users must be able to create new tasks within projects
**Priority**: High
**Acceptance Criteria**:
- A task MUST have a unique identifier (UUID)
- A task MUST have a non-empty title (1-200 characters)
- A task MAY have a description (0-1000 characters)
- A task MUST belong to exactly one project
- A task MAY be assigned to a user
- A task MAY have a due date
- A task MUST have creation and last updated timestamps
- A task MUST track who created it (created_by User ID)
- A task MUST track who last updated it (updated_by User ID)
- A task MUST start with "Todo" status and "Medium" priority by default

### 2.2 Task Information Management (FR-T-002)
**Description**: Users must be able to update task information
**Priority**: High
**Acceptance Criteria**:
- Authorized users MUST be able to update task title
- Authorized users MUST be able to update task description
- System MUST update the "updated_at" timestamp on any modification
- System MUST update the "updated_by" field with the user making the change
- Changes MUST be validated before saving

### 2.3 Task Status Management (FR-T-003)
**Description**: Users must be able to manage task workflow
**Priority**: High
**Acceptance Criteria**:
- Task status MUST be one of: Todo, InProgress, Done, Blocked
- Status transitions SHOULD follow workflow rules
- Status changes MUST update the "updated_at" timestamp
- Status changes MUST update the "updated_by" field with the user making the change
- Completed tasks (Done) SHOULD be immutable except for comments

### 2.4 Task Priority Management (FR-T-004)
**Description**: Users must be able to set task importance levels
**Priority**: Medium
**Acceptance Criteria**:
- Task priority MUST be one of: Low, Medium, High, Urgent
- Priority changes MUST update the "updated_at" timestamp
- Priority changes MUST update the "updated_by" field with the user making the change
- System SHOULD support priority-based sorting and filtering

### 2.5 Task Assignment (FR-T-005)
**Description**: Users must be able to assign tasks to team members
**Priority**: High
**Acceptance Criteria**:
- Tasks MAY be assigned to zero or one user
- Assignment changes MUST update the "updated_at" timestamp
- Assignment changes MUST update the "updated_by" field with the user making the change
- Assignees MUST receive notifications of assignment
- Only project members CAN be assigned to tasks

### 2.6 Task Due Date Management (FR-T-006)
**Description**: Users must be able to set and track task deadlines
**Priority**: Medium
**Acceptance Criteria**:
- Tasks MAY have an optional due date
- System MUST identify overdue tasks (past due date and not completed)
- Due date changes MUST update the "updated_at" timestamp
- Due date changes MUST update the "updated_by" field with the user making the change
- System SHOULD send notifications before due dates

### 2.7 Task Validation (FR-T-007)
**Description**: System must validate all task data
**Priority**: High
**Acceptance Criteria**:
- Task title MUST NOT be empty or only whitespace
- Task title MUST NOT exceed 200 characters
- Task description MUST NOT exceed 1000 characters if provided
- Project ID MUST reference an existing project
- Assignee ID MUST reference an existing user if provided
- Created_by ID MUST be a valid UUID
- Updated_by ID MUST be a valid UUID
- Due date MUST be a valid future date if provided

### 2.8 Task Query and Display (FR-T-008)
**Description**: Users must be able to view and search tasks
**Priority**: High
**Acceptance Criteria**:
- Tasks MUST display title, status, priority, and assignee
- Tasks MUST show due date and overdue status if applicable
- Tasks MUST show who created and last updated the task
- System MUST support filtering by status, priority, assignee, project, creator, updater
- System MUST support searching by title and description
- System MUST support sorting by priority, due date, creation date

### 2.9 Task Audit Trail (FR-T-009)
**Description**: System must maintain complete audit trail for tasks
**Priority**: High
**Acceptance Criteria**:
- System MUST track who created each task
- System MUST track who made each modification
- All update operations MUST record the user making the change
- Audit information MUST be immutable once recorded
- System MUST support querying tasks by creator or modifier
- Task assignment changes MUST be auditable

## 3. Non-Functional Requirements

### 3.1 Performance (NFR-T-001)
- Task creation MUST complete within 100ms
- Task queries MUST return results within 200ms
- Task list views MUST load within 500ms for up to 1000 tasks
- System MUST support 10,000+ concurrent task operations

### 3.2 Data Integrity (NFR-T-002)
- All task data MUST be validated before persistence
- Task-project relationships MUST maintain referential integrity
- Task assignments MUST reference valid users
- Orphaned tasks MUST be prevented

### 3.3 Security (NFR-T-003)
- Only authenticated users CAN create tasks
- Only project members CAN view project tasks
- Only assignees or project owners CAN modify task status
- All task operations MUST be auditable with user tracking
- System MUST maintain audit trail of who created and modified tasks

### 3.4 Usability (NFR-T-004)
- Task creation MUST be quick and intuitive
- Task status changes MUST be simple (drag-drop, click)
- Overdue tasks MUST be visually prominent
- Task lists MUST be responsive and mobile-friendly

### 3.5 Scalability (NFR-T-005)
- System MUST handle projects with 10,000+ tasks
- Task queries MUST remain performant at scale
- Task notifications MUST be efficiently processed

## 4. Business Rules

### 4.1 Task Lifecycle (BR-T-001)
- Tasks start as "Todo" by default
- Todo → InProgress → Done (normal workflow)
- Any status can transition to "Blocked"
- Blocked tasks can return to previous status when unblocked
- Done tasks should not change status (except for corrections)

### 4.2 Task Assignment Rules (BR-T-002)
- Tasks can be unassigned (assignee_id = None)
- Only one user can be assigned per task
- Assignees must be project team members
- Assignment changes should notify relevant parties

### 4.3 Task Dependencies (BR-T-003)
- Tasks belong to exactly one project
- Tasks cannot exist without a parent project
- Deleting a project should handle associated tasks appropriately

### 4.4 Due Date Rules (BR-T-004)
- Due dates are optional
- Past due dates with incomplete status = overdue
- Due dates should consider business hours and holidays
- Due date extensions should be logged

## 5. Data Model

```rust
pub struct Task {
    pub id: Uuid,                      // Unique identifier
    pub title: String,                 // 1-200 characters
    pub description: Option<String>,   // 0-1000 characters
    pub status: TaskStatus,            // Todo|InProgress|Done|Blocked
    pub priority: TaskPriority,        // Low|Medium|High|Urgent
    pub project_id: Uuid,             // Reference to Project
    pub assignee_id: Option<Uuid>,    // Reference to User (optional)
    pub due_date: Option<DateTime<Utc>>, // Optional deadline
    pub created_at: DateTime<Utc>,    // Creation timestamp
    pub updated_at: DateTime<Utc>,    // Last modification timestamp
    pub created_by: Uuid,             // Who created this task
    pub updated_by: Uuid,             // Who last updated this task
}

pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Blocked,
}

pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}
```

## 6. API Requirements

### 6.1 Create Task
- **Endpoint**: POST /projects/{project_id}/tasks
- **Input**: title, description?, assignee_id?, due_date?, priority?, created_by
- **Output**: Created task with generated ID, timestamps, and audit fields
- **Status Codes**: 201 Created, 400 Bad Request, 401 Unauthorized, 404 Not Found

### 6.2 Update Task
- **Endpoint**: PUT /tasks/{id}
- **Input**: title?, description?, status?, priority?, assignee_id?, due_date?, updated_by
- **Output**: Updated task with updated timestamp and audit fields
- **Status Codes**: 200 OK, 400 Bad Request, 403 Forbidden, 404 Not Found

### 6.3 Get Task
- **Endpoint**: GET /tasks/{id}
- **Output**: Task details
- **Status Codes**: 200 OK, 404 Not Found

### 6.4 List Tasks
- **Endpoint**: GET /projects/{project_id}/tasks
- **Query Parameters**: status?, priority?, assignee_id?, created_by?, updated_by?, overdue?, search?
- **Output**: List of tasks with pagination and audit information
- **Status Codes**: 200 OK, 404 Not Found

### 6.5 Assign Task
- **Endpoint**: PATCH /tasks/{id}/assign
- **Input**: assignee_id?, updated_by
- **Output**: Updated task with updated timestamp and audit fields
- **Status Codes**: 200 OK, 400 Bad Request, 403 Forbidden, 404 Not Found

## 7. Test Scenarios

### 7.1 Valid Task Creation
- Create task with minimal required fields
- Create task with all optional fields
- Verify default values are set correctly
- Verify timestamps are current

### 7.2 Invalid Task Creation
- Attempt creation with empty title (should fail)
- Attempt creation with oversized title (should fail)
- Attempt creation with invalid project (should fail)
- Attempt creation with invalid assignee (should fail)

### 7.3 Task Status Workflows
- Test all valid status transitions
- Test business rule enforcement
- Verify timestamp updates
- Test notification triggers

### 7.4 Due Date Management
- Create tasks with future due dates
- Test overdue detection logic
- Test due date modifications
- Verify overdue notifications

### 7.5 Task Assignment
- Assign tasks to valid users
- Unassign tasks (set to None)
- Test invalid assignment attempts
- Verify assignment notifications

### 7.6 Task Validation
- Test all validation rules
- Verify error messages are clear
- Test edge cases and boundary conditions

## 8. Integration Requirements

### 8.1 Project Integration
- Tasks must validate project existence
- Task counts must update project statistics
- Project deletion must handle tasks appropriately

### 8.2 User Integration
- Task assignments must validate user existence
- User deletion must handle assigned tasks
- User permissions must control task operations

### 8.3 Notification Integration
- Task assignments must trigger notifications
- Due date reminders must be scheduled
- Status changes must notify stakeholders

## 9. Constraints and Assumptions

### 9.1 Technical Constraints
- Task IDs must be UUIDs for distributed system compatibility
- All timestamps must be UTC to avoid timezone issues
- Text fields must support Unicode for internationalization
- Due dates must handle timezone conversions

### 9.2 Business Assumptions
- Tasks represent atomic work units
- Task complexity varies widely (5 min to several days)
- Most projects have 10-100 tasks
- Task reassignment is common during project execution
- Due dates are guidelines, not hard deadlines

### 9.3 Operational Assumptions
- Users will primarily interact via web/mobile interfaces
- Bulk task operations will be needed for large projects
- Task history and audit trails are important for accountability
- Integration with external calendar systems may be required