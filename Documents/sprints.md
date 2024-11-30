# Jira Task Creation Guide for Neurolov Development

## Step 1: Create Epics

Create the following epics first:

1. **Database Development [DB]**
   ```
   Key: ND-3
   Summary: Database Architecture
   Description: Complete database architecture implementation for Neurolov platform
   Assignee: [Database Team Lead]
   ```

2. **Website Development [WEB]**
   ```
   Key: ND-6
   Summary: Website Development: Orlov
   Description: Complete website development and deployment
   Assignee: [Frontend Team Lead]
   ```

3. **Authentication System [AUTH]**
   ```
   Key: ND-7
   Summary: Authentication System: Compute App
   Description: Implement complete authentication system for compute application
   Assignee: Dattaraj
   ```

4. **Core API Development [API]**
   ```
   Key: ND-8
   Summary: Core API Development: Compute App
   Description: Develop and implement core API infrastructure
   Assignee: Gokul
   ```

5. **Telegram App Development [TG]**
   ```
   Key: ND-9
   Summary: Telegram App v2
   Description: Development of Telegram v2 application
   Assignee: [Telegram Team Lead]
   ```

## Step 2: Create Stories Under Each Epic

### Database Architecture (ND-3)
```
Story 1: Database Schema Design
- Priority: High
- Story Points: 8
- Assignee: [Database Team]
- Subtasks:
  * Create ERD
  * Define relationships
  * Document schema

Story 2: Database Implementation
- Priority: High
- Story Points: 13
- Assignee: [Database Team]
- Subtasks:
  * Set up database server
  * Implement migrations
  * Create backup system
```

### Website Development (ND-6)
```
Story 1: Frontend Component Development
- Priority: High
- Story Points: 8
- Assignee: Dattaraj
- Subtasks:
  * Create reusable components
  * Implement routing
  * Set up state management

Story 2: Integration with Backend
- Priority: Medium
- Story Points: 5
- Assignee: Dattaraj
- Subtasks:
  * API integration
  * Error handling
  * Testing
```

### Authentication System (ND-7)
```
Story 1: SSO Implementation
- Priority: High
- Story Points: 8
- Assignee: Dattaraj
- Subtasks:
  * OAuth setup
  * Token management
  * Session handling

Story 2: User Management
- Priority: Medium
- Story Points: 5
- Assignee: Nithish
- Subtasks:
  * User profile
  * Permission system
  * Account settings
```

### Core API Development (ND-8)
```
Story 1: API Architecture
- Priority: High
- Story Points: 13
- Assignee: Gokul
- Subtasks:
  * Design API structure
  * Create documentation
  * Implement base classes

Story 2: Resource Management
- Priority: High
- Story Points: 8
- Assignee: Gokul
- Subtasks:
  * GPU resource allocation
  * Load balancing
  * Monitoring system
```

### Telegram App v2 (ND-9)
```
Story 1: Bot Framework Setup
- Priority: Medium
- Story Points: 5
- Assignee: [Telegram Team]
- Subtasks:
  * Bot configuration
  * Command handling
  * User interaction

Story 2: Feature Implementation
- Priority: Medium
- Story Points: 8
- Assignee: [Telegram Team]
- Subtasks:
  * User commands
  * Notifications
  * Analytics
```

## Step 3: Task Creation Rules

1. **Naming Convention**
```
[Project Key]-[Epic Key]-[Task Type]: Brief Description
Example: ND-3-DB-TASK: Implement User Schema
```

2. **Required Fields**
```
- Summary: Clear, concise description
- Priority: High/Medium/Low
- Assignee: Team member responsible
- Story Points: 1, 2, 3, 5, 8, 13 (Fibonacci)
- Sprint: Current sprint number
- Epic Link: Parent epic
- Labels: feature/bug/documentation
```

3. **Description Template**
```
Objective:
[Clear statement of what needs to be done]

Requirements:
- [Specific requirement 1]
- [Specific requirement 2]

Acceptance Criteria:
- [ ] Criteria 1
- [ ] Criteria 2

Dependencies:
- [List any blocking tasks]

Definition of Done:
- [ ] Code complete
- [ ] Tests written
- [ ] Documentation updated
- [ ] PR reviewed
- [ ] Deployed to staging
```

## Step 4: Setting Up Sprints

1. **Sprint Creation**
```
Name: Sprint [Number] - [Start Date] to [End Date]
Duration: 2 weeks
Start Date: Monday
End Date: Friday (following week)
```

2. **Sprint Planning**
```
- Prioritize tasks based on dependencies
- Assign story points
- Set sprint goals
- Schedule daily standups
```

## Step 5: Task Tracking

1. **Status Updates**
```
- To Do: Not started
- In Progress: Currently being worked on
- Code Review: Ready for review
- Testing: In QA
- Done: Completed and verified
```

2. **Daily Updates**
```
- Update task status
- Log time spent
- Add comments for blockers
- Link related PRs
```

## Important Notes:

1. **Task Creation Checklist**
- [ ] Linked to correct epic
- [ ] Properly assigned
- [ ] Story points set
- [ ] Priority defined
- [ ] Labels added
- [ ] Dependencies identified
- [ ] Subtasks created if needed

2. **Best Practices**
- Update tasks daily
- Add comments for clarity
- Link related work (PRs, documents)
- Follow naming conventions
- Keep descriptions clear and concise

3. **Common Labels**
- feature
- bug
- documentation
- enhancement
- blocker
- technical-debt
- 
