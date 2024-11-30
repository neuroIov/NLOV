# Daily Recurring Tasks - Jira Setup Guide

## Step 1: Create Daily Task Epic

```
Epic Name: Daily Recurring Tasks
Key: ND-DAILY
Description: Tracking daily recurring tasks across all teams
Status: Ongoing
```

## Step 2: Create Parent Tasks

Create these as parent tasks that will contain daily subtasks:

### Marketing & Content (ND-DAILY-MKT)
```
1. Social Media Management
   Assignee: Sparsh
   Subtasks:
   - Daily Tweet Planning
   - Quote Tweets
   - Retweets
   Status: Repeating Daily

2. Content Creation
   Assignee: Dhruvanshi
   Subtasks:
   - Medium Article
   - PR Article Draft
   Status: Repeating Daily

3. Community Management
   Assignee: Sanskar
   Subtasks:
   - Telegram Announcements
   - Discord Updates
   - Event Coordination
   Status: Repeating Daily
```

### Business Development (ND-DAILY-BIZ)
```
1. Partnership Development
   Assignees: Sparsh, Ruzaliya
   Subtasks:
   - Partnership Outreach (5 daily)
   - Integration Discussions
   - Follow-up Calls
   Status: Repeating Daily

2. Investor Relations
   Assignees: Ruzaliya, Alina, Claire, Umar
   Subtasks:
   - Investor Outreach (10 daily)
   - Meeting Coordination
   - Follow-up Communication
   Status: Repeating Daily

3. Advisor Management
   Assignee: Ruzaliya
   Subtasks:
   - Advisor Outreach (3 daily)
   - Meeting Scheduling
   - Relationship Management
   Status: Repeating Daily
```

### Development Tasks (ND-DAILY-DEV)
```
1. Carnival Development
   Assignee: Nithish
   Subtasks:
   - Feature Development
   - Bug Fixes
   - Status Updates
   Status: Repeating Daily

2. T2E Development
   Assignees: Vaibav, Gokul
   Subtasks:
   - Development Progress
   - Integration Testing
   - Documentation
   Status: Repeating Daily

3. Compute App Development
   Assignees: Dattaraj, Nithish, Gokul
   Subtasks:
   - Feature Implementation
   - Code Review
   - Testing
   Status: Repeating Daily
```

## Step 3: Automation Rules

### Daily Task Creation
```yaml
Trigger: Schedule (Daily at 12:00 AM)
Action: 
  - Create subtasks for each parent task
  - Set due date to end of day
  - Assign to respective team members
```

### Status Updates
```yaml
Trigger: Time-based (Every 4 hours)
Action:
  - Send reminder if task not updated
  - Update dashboard metrics
  - Notify team lead of pending items
```

### Task Completion
```yaml
Trigger: Task marked as Done
Action:
  - Update completion metrics
  - Add to daily report
  - Create next day's task
```

## Step 4: Dashboard Setup

### Daily Progress Dashboard
```
Widgets:
1. Tasks Completed Today
2. Tasks In Progress
3. Pending Tasks
4. Team Member Performance
5. Completion Rate Trends
```

### Team Performance Metrics
```
Track:
- Tasks completed on time
- Response time to assignments
- Quality of deliverables
- Team collaboration
```

## Step 5: Daily Reporting

### Morning Report (9 AM)
```
- Tasks carried over
- Today's priorities
- Resource allocation
- Expected deliverables
```

### Evening Report (6 PM)
```
- Tasks completed
- Blockers encountered
- Solutions implemented
- Next day preparation
```

## Step 6: Jira Task Template

```
Title: [Team] - [Task Name] - [Date]
Example: "MKT - Daily Tweets - 2024-11-30"

Description:
Objective: [Clear description of daily task]
Requirements:
- [Specific requirement 1]
- [Specific requirement 2]

Checklist:
- [ ] Task initiated
- [ ] Work in progress
- [ ] Review completed
- [ ] Task finalized

Updates:
- Time Started: [HH:MM]
- Status Updates: [Add throughout day]
- Completion Time: [HH:MM]

Additional Notes:
- [Any relevant information]
- [Blockers encountered]
- [Solutions implemented]
```

## Step 7: Workflow States

```
1. To Do
2. In Progress
3. Review Required
4. Completed
5. Verified
```

## Important Notes:

1. Task Creation Rules:
   - Create tasks at start of day
   - Set proper assignees
   - Include all required subtasks
   - Link to relevant parent tasks

2. Status Update Requirements:
   - Update status minimum 3 times daily
   - Add blockers immediately
   - Document all key decisions
   - Link related work items

3. Reporting Requirements:
   - Daily completion status
   - Weekly performance metrics
   - Monthly trend analysis
   - Team productivity reports

4. Best Practices:
   - Update tasks in real-time
   - Use standardized labels
   - Follow naming conventions
   - Maintain clear documentation

5. Quick Filters for Teams:
   - Marketing Tasks
   - Development Tasks
   - Business Development Tasks
   - Management Overview
