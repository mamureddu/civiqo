# Agent 2 Review Workflow

**Description**: Standalone Agent 2 (Tech Lead) technical review for existing PRs or feature branches. Used when Agent 1 implementation is complete and ready for verification.

**Usage**: `/agent-2-review`

---

## 🔍 Agent 2 Technical Review Process

### Step 1: PR Analysis
- Review the PR description and implementation plan
- Examine the scope and complexity of changes
- Identify affected files and components
- Check for any breaking changes or deprecations
- Verify manual testing checklist is provided

### Step 2: Security Assessment
- **Authentication**: Verify AuthUser extractors are properly used
- **Authorization**: Check permission controls and access restrictions
- **SQL Injection**: Confirm all queries use parameterized bindings
- **Input Validation**: Review sanitization and validation logic
- **Session Management**: Assess session security and handling
- **Data Exposure**: Check for sensitive data leaks

### Step 3: Code Quality Review
- **Architecture**: Verify MVC pattern adherence
- **Structure**: Assess code organization and modularity
- **Error Handling**: Review comprehensive error management
- **Logging**: Check appropriate logging for debugging
- **Performance**: Identify potential bottlenecks or inefficiencies
- **Maintainability**: Evaluate code readability and documentation

### Step 4: Brand Compliance Verification
- **UI Components**: Verify Civiqo brand assets usage
- **Styling**: Check TailwindCSS consistency and color schemes
- **Typography**: Validate font usage and hierarchy
- **Layout**: Review responsive design implementation
- **Accessibility**: Assess WCAG compliance and usability
- **User Experience**: Evaluate overall design consistency

### Step 5: Database & Data Layer Review
- **Query Optimization**: Review SQL query efficiency
- **Transaction Safety**: Verify proper transaction handling
- **Schema Impact**: Assess database changes and migrations
- **Data Integrity**: Check foreign keys and constraints
- **Connection Management**: Review connection pooling usage
- **Error Recovery**: Verify rollback and error handling

### Step 6: Testing & Validation
- **Test Coverage**: Verify all tests are passing (baseline 189/189)
- **Regression Testing**: Check for broken existing functionality
- **Edge Cases**: Assess handling of boundary conditions
- **Integration**: Review component integration testing
- **Manual Testing**: Verify manual testing checklist completion

### Step 7: Production Readiness Assessment
- **Build Status**: Confirm zero compilation errors
- **Dependencies**: Review dependency changes and security
- **Environment Variables**: Check configuration requirements
- **Deployment Impact**: Assess deployment complexity
- **Monitoring**: Evaluate logging and observability needs
- **Rollback Plan**: Verify rollback procedures exist

### Step 8: Performance & Scalability
- **Database Queries**: Review query optimization and indexing
- **Memory Usage**: Assess potential memory leaks or bloat
- **Response Times**: Evaluate API endpoint performance
- **Concurrency**: Check thread safety and race conditions
- **Resource Utilization**: Review CPU and efficiency considerations
- **Caching**: Assess caching strategy if applicable

### Step 9: Documentation & Communication
- **Code Comments**: Review inline documentation quality
- **API Documentation**: Check endpoint documentation updates
- **README Updates**: Verify project documentation changes
- **Commit Messages**: Assess commit message clarity and format
- **PR Description**: Evaluate description completeness
- **Future Considerations**: Note any technical debt or improvements

### Step 10: Final Decision & Feedback
- **Overall Assessment**: Provide comprehensive evaluation
- **Quality Score**: Rate implementation (0-10 scale)
- **Security Rating**: Rate security posture (Low/Medium/High Risk)
- **Performance Rating**: Assess performance impact
- **Maintainability**: Evaluate long-term maintenance considerations
- **Decision**: **APPROVE** for merge or **REQUEST CHANGES**
- **Feedback**: Provide specific, actionable feedback
- **Next Steps**: Outline any additional requirements

---

## 📊 Review Scoring System

### Overall Quality Score (0-10)
- **9-10**: Exceptional - Exceeds expectations, production-ready
- **7-8**: Good - Meets all requirements, minor improvements possible
- **5-6**: Acceptable - Functional but needs improvements
- **3-4**: Needs Work - Significant issues requiring fixes
- **0-2**: Unacceptable - Major problems, cannot merge

### Security Risk Assessment
- **Low Risk**: No security concerns, best practices followed
- **Medium Risk**: Minor security issues, fixes recommended
- **High Risk**: Significant security vulnerabilities, must fix

### Performance Impact
- **Positive**: Improves performance
- **Neutral**: No performance impact
- **Negative**: Degrades performance (needs optimization)

---

## ✅ Approval Checklist

**Security Requirements:**
- [ ] Authentication properly enforced
- [ ] SQL injection protection implemented
- [ ] Input validation comprehensive
- [ ] Authorization controls appropriate
- [ ] No sensitive data exposure

**Code Quality Standards:**
- [ ] MVC pattern followed
- [ ] Error handling comprehensive
- [ ] Logging appropriate for debugging
- [ ] Code maintainable and readable
- [ ] Documentation adequate

**Brand Compliance:**
- [ ] Civiqo assets used correctly
- [ ] UI/UX consistent with design system
- [ ] Responsive design implemented
- [ ] Accessibility standards met
- [ ] Typography and colors compliant

**Testing & Validation:**
- [ ] All tests passing (189/189+)
- [ ] No regressions detected
- [ ] Manual testing completed
- [ ] Edge cases handled
- [ ] Integration verified

**Production Readiness:**
- [ ] Zero compilation errors
- [ ] Deployment compatible
- [ ] Environment variables documented
- [ ] Monitoring/logging adequate
- [ ] Rollback procedures available

---

## 🔄 Review Outcomes

### **APPROVED** - Ready for Merge
- All requirements met
- Quality score 7+
- Security risk Low
- No blocking issues

### **APPROVED WITH RECOMMENDATIONS** - Merge with Future Improvements
- Core functionality solid
- Minor improvements suggested
- Document recommendations for next iteration

### **CHANGES REQUESTED** - Not Ready for Merge
- Blocking issues identified
- Security concerns present
- Quality below standards
- Specific fixes required

---

## 📝 Review Report Template

```markdown
# Agent 2 Technical Review Report

## PR Information
- **PR #**: [Number]
- **Feature**: [Feature Name]
- **Files Changed**: [List]
- **Lines Added/Removed**: [+X/-Y]

## Assessment Summary
- **Overall Quality**: [X]/10
- **Security Risk**: [Low/Medium/High]
- **Performance Impact**: [Positive/Neutral/Negative]
- **Decision**: [APPROVED/CHANGES REQUESTED]

## Detailed Findings
### Security
- [Findings]

### Code Quality
- [Findings]

### Brand Compliance
- [Findings]

### Testing
- [Findings]

### Production Readiness
- [Findings]

## Recommendations
- [Specific recommendations]

## Next Steps
- [Required actions if changes requested]
```

---

*This workflow ensures thorough technical review while maintaining our high quality standards and security posture.*
