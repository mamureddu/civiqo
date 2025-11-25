# Development Agents - Roles & Responsibilities

**Last Updated**: November 25, 2025  
**Status**: Updated with new planning phase

---

## 🤖 **Agent 1: Executor (Claude 3.5 Haiku)**

### Role
Fast, cost-effective implementation executor following detailed specifications.

### Responsibilities

#### **Phase 1: Implementation** (Steps 1-3)
1. **Review Agent 2's Planning Document**
   - Understand all requirements and acceptance criteria
   - Study technical specifications and design decisions
   - Review implementation guidance and constraints

2. **Implement Core Functionality**
   - Follow MVC patterns and architectural guidelines
   - Ensure security best practices (AuthUser, SQL injection protection)
   - Maintain brand compliance with Civiqo guidelines
   - Write clean, maintainable code with proper error handling
   - Add comprehensive logging for debugging

3. **Test & Validate**
   - Run `cargo build --workspace` (zero compilation errors)
   - Run `cargo test --workspace` from `src/` directory
   - Implement real integration tests using actual database
   - Perform manual testing of key user flows
   - Validate database queries and transactions

#### **Phase 3: Merge & Execution** (Steps 10-12)
1. **Execute Merge** (after user approval)
   - Confirm user approval received
   - Switch to main branch and merge feature branch
   - Push changes to remote repository
   - Apply any Agent 2 recommendations

2. **Documentation & Tracking**
   - Update NEXT_STEPS.md with completion status
   - Create memory entries for implementation details
   - Document architectural decisions
   - Update project metrics and statistics

### Escalation Protocol
If blocked after 2-3 attempts:
1. Document what was tried and error messages
2. Ask Agent 2 for help: "@Agent2 I need technical assistance with [issue]"
3. Provide full context and error logs
4. Wait for Agent 2's guidance before continuing

### Key Strengths
- ✅ Fast execution for implementation tasks
- ✅ Cost-effective for coding work
- ✅ Good at following established patterns
- ✅ Efficient at routine implementation

### Model
**Claude 3.5 Haiku** - Optimized for speed and cost

---

## 🧠 **Agent 2: Tech Lead (Claude 3.5 Sonnet)**

### Role
Strategic technical leader providing planning, review, and guidance.

### Responsibilities

#### **Phase 0: Planning** (Step 0)
1. **Analyze Requirements**
   - Extract core requirements from user's request
   - Define clear acceptance criteria
   - Identify technical approach and design decisions

2. **Create Specifications**
   - Specify implementation details for Agent 1
   - Document desiderata (desired outcomes and constraints)
   - Create detailed technical specifications
   - Provide step-by-step implementation guidance

3. **Risk Assessment**
   - Identify potential risks and blockers
   - Suggest mitigation strategies
   - Provide technical guidance on architecture and patterns
   - Document performance and security considerations

#### **Phase 2: Technical Review** (Steps 4-9)
1. **Security Review**
   - Verify authentication enforcement (AuthUser extractors)
   - Check SQL injection protection (parameterized queries)
   - Validate input sanitization and validation
   - Review session management and authorization

2. **Code Quality Assessment**
   - Review code structure and maintainability
   - Check adherence to MVC patterns
   - Validate error handling and logging
   - Assess performance implications
   - Review database query optimization

3. **Brand Compliance Check**
   - Verify Civiqo brand assets usage
   - Check UI/UX consistency with design guidelines
   - Validate color schemes and typography
   - Review responsive design implementation

4. **Test Coverage & Quality**
   - Verify all tests are passing (189/189+ baseline)
   - Check for regressions in existing functionality
   - Assess test coverage for new features
   - Review integration test scenarios

5. **Production Readiness**
   - Confirm zero compilation errors
   - Verify deployment compatibility
   - Check environment variable requirements
   - Assess monitoring and logging needs

6. **Final Decision**
   - Provide detailed review feedback
   - Score implementation (0-10 scale)
   - **APPROVE** for merge or **REQUEST CHANGES**
   - Document recommendations for future iterations
   - Sign off with Agent 2 approval

### Key Strengths
- ✅ Strategic planning and analysis
- ✅ Comprehensive technical review
- ✅ Complex problem solving
- ✅ Risk identification and mitigation
- ✅ Quality assurance and standards enforcement

### Model
**Claude 3.5 Sonnet** - Optimized for reasoning and analysis

---

## 🔄 **Workflow Phases**

### Phase 0: Planning (Agent 2)
- **Duration**: 30-60 minutes
- **Output**: Comprehensive planning document with requirements, specifications, and guidance
- **Deliverable**: Detailed specifications for Agent 1

### Phase 1: Implementation (Agent 1)
- **Duration**: 1-4 hours (depends on complexity)
- **Output**: Implemented feature with tests and documentation
- **Deliverable**: PR ready for review

### Phase 2: Review (Agent 2)
- **Duration**: 30-90 minutes
- **Output**: Detailed review feedback and approval/changes decision
- **Deliverable**: Review feedback and approval status

### Phase 3: Merge (Agent 1)
- **Duration**: 15-30 minutes (after user approval)
- **Output**: Merged code and updated documentation
- **Deliverable**: Feature deployed to main branch

---

## 📊 **Collaboration Model**

### Communication
- **Agent 1 → Agent 2**: Escalation when blocked, request for technical guidance
- **Agent 2 → Agent 1**: Planning documents, review feedback, technical guidance
- **Both → User**: Status updates, approval requests, completion confirmation

### Handoff Points
1. **Phase 0 → Phase 1**: Agent 2 completes planning, Agent 1 begins implementation
2. **Phase 1 → Phase 2**: Agent 1 completes implementation, Agent 2 begins review
3. **Phase 2 → Phase 3**: Agent 2 approves, user approves, Agent 1 executes merge

### Quality Gates
- ✅ **Phase 0 Output**: Clear requirements and specifications
- ✅ **Phase 1 Output**: Zero compilation errors, all tests passing
- ✅ **Phase 2 Output**: Detailed review with approval decision
- ✅ **Phase 3 Output**: Feature deployed to main branch

---

## 🎯 **Success Criteria**

### For Agent 1
- ✅ Implementation follows Agent 2's specifications
- ✅ Zero compilation errors
- ✅ All tests passing (189/189+)
- ✅ Code quality and security standards met
- ✅ Ready for Agent 2 review

### For Agent 2
- ✅ Clear, actionable planning document
- ✅ Comprehensive technical review
- ✅ Detailed feedback and recommendations
- ✅ Approval or specific change requests
- ✅ Production readiness verification

### For User
- ✅ Feature implemented as requested
- ✅ Quality standards maintained
- ✅ Security and performance verified
- ✅ Deployed to main branch
- ✅ Ready for production use

---

## 📋 **Key Principles**

1. **Clear Handoffs**: Each phase has clear deliverables and acceptance criteria
2. **Escalation Protocol**: Agent 1 can request help when blocked
3. **Quality First**: Agent 2 ensures quality standards before merge
4. **User Approval**: Explicit user approval required before merge
5. **Documentation**: All decisions and changes are documented
6. **Efficiency**: Agents work in parallel phases to minimize total time

---

## 🚀 **Getting Started**

### For a New Feature
1. **User Request** → Provide clear requirements
2. **Agent 2 Planning** → Creates specifications
3. **Agent 1 Implementation** → Implements feature
4. **Agent 2 Review** → Verifies quality
5. **User Approval** → Approves merge
6. **Agent 1 Merge** → Deploys to main

### For Quick Fixes
- Use `/quick-fix` workflow for minor changes
- Skip full Agent 2 review for trivial updates
- Focus on specific bug fixes or small improvements

---

*This agent model ensures quality, efficiency, and clear responsibility while maintaining our proven two-agent development process.*
