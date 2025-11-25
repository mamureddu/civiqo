# Agents Memory - Shared Context During Task Execution

**Purpose**: Maintain shared context and state during complex multi-phase tasks without cluttering the main codebase.

## Overview

This folder is used by Agent 1 and Agent 2 to maintain working documents and context during task execution. Files here are:
- **Temporary**: Created at task start, cleaned up at task completion
- **Shared**: Accessible to both agents for context continuity
- **Focused**: Task-specific, not general documentation
- **Clean**: Removed after task completion to avoid clutter

## File Structure

### During Task Execution

```
agents_memory/
├── TASK_CONTEXT.md          # Current task overview and requirements
├── IMPLEMENTATION_PLAN.md   # Detailed implementation steps and progress
├── BLOCKERS_AND_NOTES.md    # Issues, blockers, and decision notes
└── TESTING_CHECKLIST.md     # Testing requirements and progress
```

## File Descriptions

### TASK_CONTEXT.md
**Created by**: Agent 2 (Planning phase)  
**Updated by**: Both agents  
**Contains**:
- Task overview and objectives
- Acceptance criteria
- Key requirements and constraints
- Technical approach summary
- Risk assessment

**Example**:
```markdown
# Task: Community CRUD Routes Implementation

## Objectives
- Implement POST /api/communities (Create)
- Implement PUT /api/communities/:id (Update)
- Implement DELETE /api/communities/:id (Delete)

## Acceptance Criteria
- [ ] All endpoints working
- [ ] Tests passing
- [ ] Security verified
- [ ] Documentation complete

## Key Requirements
- Owner-only permission checks
- Proper error handling
- Database transactions
- Input validation
```

### IMPLEMENTATION_PLAN.md
**Created by**: Agent 2 (Planning phase)  
**Updated by**: Agent 1 (Implementation phase)  
**Contains**:
- Step-by-step implementation tasks
- Progress tracking
- Technical specifications
- Code structure decisions
- Testing requirements

**Example**:
```markdown
# Implementation Plan

## Phase 1: Create Community (POST)
- [ ] Create handler function
- [ ] Add input validation
- [ ] Implement database insert
- [ ] Add error handling
- [ ] Write tests

## Phase 2: Update Community (PUT)
- [ ] Create handler function
- [ ] Add permission checks
- [ ] Implement database update
- [ ] Add error handling
- [ ] Write tests

## Phase 3: Delete Community (DELETE)
- [ ] Create handler function
- [ ] Add permission checks
- [ ] Implement cascade delete
- [ ] Add error handling
- [ ] Write tests
```

### BLOCKERS_AND_NOTES.md
**Created by**: Agent 1 (Implementation phase)  
**Updated by**: Both agents  
**Contains**:
- Blockers and issues encountered
- Decision notes and rationale
- Technical challenges and solutions
- Questions for Agent 2
- Important findings

**Example**:
```markdown
# Blockers and Notes

## Issues Encountered
- Issue 1: Database constraint conflict
  - Solution: Use ON CONFLICT clause
  - Status: RESOLVED

## Technical Decisions
- Decision 1: Use transaction for cascade delete
  - Rationale: Ensure data consistency
  - Approved by: Agent 2

## Questions for Agent 2
- Q1: Should we implement soft delete?
  - Status: PENDING RESPONSE

## Important Findings
- Finding 1: Existing indexes cover new queries
  - Impact: No new indexes needed
```

### TESTING_CHECKLIST.md
**Created by**: Agent 2 (Planning phase)  
**Updated by**: Agent 1 (Implementation phase)  
**Contains**:
- Unit test requirements
- Integration test requirements
- Manual testing checklist
- Test progress and results
- Coverage metrics

**Example**:
```markdown
# Testing Checklist

## Unit Tests
- [ ] Create community validation
- [ ] Update community validation
- [ ] Delete community validation
- [ ] Permission checks

## Integration Tests
- [ ] POST /api/communities endpoint
- [ ] PUT /api/communities/:id endpoint
- [ ] DELETE /api/communities/:id endpoint
- [ ] Error scenarios

## Manual Testing
- [ ] Create community via UI
- [ ] Update community via UI
- [ ] Delete community via UI
- [ ] Permission enforcement

## Coverage
- Target: 80%+
- Current: [to be updated]
```

## Workflow Integration

### Phase 0: Planning (Agent 2)
1. Create `TASK_CONTEXT.md` with requirements
2. Create `IMPLEMENTATION_PLAN.md` with steps
3. Create `TESTING_CHECKLIST.md` with test requirements
4. Create `BLOCKERS_AND_NOTES.md` (empty, for Agent 1 to use)

### Phase 1: Implementation (Agent 1)
1. Review all memory files
2. Update `IMPLEMENTATION_PLAN.md` with progress
3. Update `BLOCKERS_AND_NOTES.md` with issues
4. Update `TESTING_CHECKLIST.md` with test results

### Phase 2: Review (Agent 2)
1. Review memory files for context
2. Update `BLOCKERS_AND_NOTES.md` with feedback
3. Provide guidance on blockers

### Phase 3: Cleanup (Agent 1)
1. Verify task completion
2. Delete all memory files
3. Commit cleanup

## Best Practices

### For Agent 2 (Planning)
- ✅ Create clear, actionable tasks
- ✅ Document all requirements upfront
- ✅ Identify potential blockers
- ✅ Provide implementation guidance

### For Agent 1 (Implementation)
- ✅ Update progress regularly
- ✅ Document blockers immediately
- ✅ Ask questions in BLOCKERS_AND_NOTES.md
- ✅ Track test progress

### For Both Agents
- ✅ Keep files concise and focused
- ✅ Use checkboxes for progress tracking
- ✅ Update timestamps for clarity
- ✅ Clean up after task completion

## Cleanup Process

### When Task is Complete
1. **Verify Completion**
   - All acceptance criteria met
   - All tests passing
   - Code merged to main

2. **Delete Memory Files**
   ```bash
   rm agents_memory/TASK_CONTEXT.md
   rm agents_memory/IMPLEMENTATION_PLAN.md
   rm agents_memory/BLOCKERS_AND_NOTES.md
   rm agents_memory/TESTING_CHECKLIST.md
   ```

3. **Commit Cleanup**
   ```bash
   git add -A
   git commit -m "cleanup: Remove agents_memory files for completed task"
   git push origin main
   ```

## Example Usage

### Starting a Task
```bash
# Agent 2 creates memory files
# Agent 1 reviews and starts implementation
# Both update files during execution
# Agent 1 deletes files when complete
```

### During Implementation
```markdown
# IMPLEMENTATION_PLAN.md
## Phase 1: Create Community (POST)
- [x] Create handler function
- [x] Add input validation
- [x] Implement database insert
- [ ] Add error handling
- [ ] Write tests

# BLOCKERS_AND_NOTES.md
## Issues Encountered
- Issue 1: Database constraint conflict
  - Solution: Use ON CONFLICT clause
  - Status: RESOLVED

## Questions for Agent 2
- Q1: Should we implement soft delete?
  - Status: PENDING RESPONSE
```

## Benefits

1. **Context Continuity**
   - Shared understanding between agents
   - No context loss between phases
   - Clear task state at all times

2. **No Clutter**
   - Temporary files deleted after task
   - Main codebase stays clean
   - Easy to find completed work

3. **Better Collaboration**
   - Clear communication channel
   - Documented decisions
   - Tracked progress

4. **Efficient Debugging**
   - Blockers documented
   - Solutions tracked
   - Decisions explained

## Important Notes

- **Do NOT commit memory files to main branch** (except cleanup commit)
- **Do NOT use for general documentation** (use `docs/` for that)
- **Do NOT leave files after task completion** (clean up immediately)
- **Do use for complex, multi-phase tasks** (simple tasks may not need all files)

---

*This system keeps agents synchronized during complex tasks while maintaining a clean, organized codebase.*
