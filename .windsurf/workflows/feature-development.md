# Feature Development Workflow

**Description**: Complete feature development cycle with Agent 1 (Executor) implementation and Agent 2 (Tech Lead) verification. Maintains our proven two-agent quality process while providing structured automation.

**Usage**: `/feature-development`

---

## 🤖 **IMPORTANT: Model Selection**

**Before starting, manually select the appropriate model:**

### Agent 1 (Executor) - Steps 1-4
👉 **Use Model**: Claude 3.5 Haiku
- Faster execution for implementation
- Cost-effective for coding tasks
- Good for following established patterns

### Agent 2 (Tech Lead) - Steps 5-8
👉 **Use Model**: Claude 3.5 Sonnet
- Better analysis and reasoning
- Comprehensive technical review
- Complex problem solving

**Note**: Automatic model selection is not yet supported by Windsurf workflows. This is a manual step until the feature is available.

---

## 🚀 Phase 1: Agent 1 (Executor) Implementation

### Step 1: Planning & Analysis
- Analyze the user's request and requirements
- Review existing codebase for context and patterns
- **Check memories and documentation** for similar issues
- Create a detailed implementation plan with todo list
- Identify potential risks and dependencies

### Step 2: Implementation
- Implement the core functionality following MVC patterns
- Ensure brand compliance with Civiqo guidelines
- Maintain security standards (AuthUser, SQL injection protection)
- Write clean, maintainable code with proper error handling
- Add comprehensive logging for debugging

**🆘 Escalation Protocol**: If you encounter a blocking issue after 2-3 attempts:
1. Document what you tried and error messages
2. Ask Agent 2 for help: "@Agent2 I need technical assistance with [issue]"
3. Provide full context and error logs
4. Wait for Agent 2's guidance before continuing

### Step 3: Testing & Validation
- Run `cargo build --workspace` to ensure zero compilation errors
- Run `cargo test --workspace` from `src/` directory (database is available)
- **IMPLEMENT real integration tests** (not stubs)
- **Use real database** for integration tests (no SQLX_OFFLINE unless needed)
- Perform manual testing of key user flows
- Check UI/UX consistency with existing patterns
- Validate database queries and transactions

### Step 4: Preparation for Review
- Commit changes with descriptive commit message
- Push to feature branch
- Create PR with detailed description
- Include manual testing checklist
- Mark implementation as ready for Agent 2 review
- **DO NOT MERGE** - wait for Agent 2 review and user approval

**🛑 STOP AND WAIT**: Agent 1 implementation complete. Switch to Agent 2 for technical review.

---

## 🔍 Phase 2: Agent 2 (Tech Lead) Verification

**👉 Switch to Claude 3.5 Sonnet model now**

### Step 5: Security Review
- Verify authentication enforcement (AuthUser extractors)
- Check SQL injection protection (parameterized queries)
- Validate input sanitization and validation
- Review session management and authorization
- Assess any potential security vulnerabilities

### Step 6: Code Quality Assessment
- Review code structure and maintainability
- Check adherence to MVC patterns
- Validate error handling and logging
- Assess performance implications
- Review database query optimization

### Step 7: Brand Compliance Check
- Verify Civiqo brand assets usage
- Check UI/UX consistency with design guidelines
- Validate color schemes and typography
- Review responsive design implementation
- Ensure accessibility standards

### Step 8: Test Coverage & Quality
- Verify all tests are passing (189/189 baseline)
- Check for any regressions in existing functionality
- Assess test coverage for new features
- Review integration test scenarios
- Validate edge case handling

### Step 9: Production Readiness
- Confirm zero compilation errors
- Verify deployment compatibility
- Check environment variable requirements
- Assess monitoring and logging needs
- Validate rollback procedures

### Step 10: Final Decision
- Provide detailed review feedback
- Score implementation (0-10 scale)
- **APPROVE** for merge or **REQUEST CHANGES**
- Document any recommendations for future iterations
- Sign off with Agent 2 approval
- **WAIT FOR USER TO EXPLICITLY APPROVE MERGE**

**🛑 STOP AND WAIT**: Agent 2 review complete. **USER MUST APPROVE MERGE BEFORE PROCEEDING**.

---

## ✅ Phase 3: Merge & Cleanup (Upon User Approval)

**⚠️ IMPORTANT**: Only proceed with this phase after receiving explicit user approval to merge.

### Step 11: Merge Process
- **Confirm user approval received** before proceeding
- Switch to main branch
- Merge feature branch (fast-forward preferred)
- Push changes to remote repository
- Apply any Agent 2 recommendations (if applicable)
- **Keep feature branch** (don't delete until user confirms)
- Update project documentation

### Step 12: Documentation & Tracking
- Update NEXT_STEPS.md with completion status
- Create memory entry for implementation details
- Document any architectural decisions
- Update project metrics and statistics
- Prepare for next development phase

### Step 13: Success Confirmation
- Confirm all systems are working post-merge
- Verify deployment pipeline compatibility
- Update project roadmap if needed
- Celebrate successful feature completion! 🎉

---

## 🔄 Alternative Paths

### If Changes Requested:
- Return to Phase 1 Step 2 with specific feedback
- Address Agent 2's concerns systematically
- Re-run testing and validation
- Resubmit for review

### Quick Fix Mode:
- Use `/quick-fix` workflow for minor changes
- Skip full Agent 2 review for trivial updates
- Focus on specific bug fixes or small improvements

---

## 📋 Quality Checklist

**Agent 1 Responsibilities:**
- [ ] Requirements fully understood
- [ ] Implementation follows MVC patterns
- [ ] Security best practices applied
- [ ] Brand compliance maintained
- [ ] All tests passing
- [ ] Manual testing completed
- [ ] Documentation updated

**Agent 2 Responsibilities:**
- [ ] Security review passed
- [ ] Code quality approved
- [ ] Brand compliance verified
- [ ] Test coverage adequate
- [ ] Production ready confirmed
- [ ] Detailed feedback provided
- [ ] Final decision documented

---

## 🎯 Success Metrics

- **Zero compilation errors**
- **All tests passing (189/189+)**
- **Security vulnerabilities: 0**
- **Brand compliance: 100%**
- **Code quality score: 8+/10**
- **Production deployment ready**

---

*This workflow maintains our proven two-agent quality process while providing structured automation through Windsurf's workflow system.*
