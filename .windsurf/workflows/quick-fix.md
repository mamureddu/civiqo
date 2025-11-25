# Quick Fix Workflow

**Description**: Fast-track workflow for minor bug fixes, small improvements, and trivial changes that don't require full Agent 2 review.

**Usage**: `/quick-fix`

---

## 🚀 Quick Implementation

### Step 1: Issue Analysis
- Identify the specific issue or improvement needed
- Verify this is a minor change (single file, <50 lines)
- Confirm no security implications
- Check no brand compliance impact
- Validate no database schema changes required

### Step 2: Implementation
- Make the minimal necessary changes
- Maintain existing code patterns and style
- Ensure no regressions in functionality
- Add appropriate error handling if needed
- Keep changes focused and scoped

### Step 3: Quick Validation
- Run `cargo build --workspace` (must pass)
- Run `SQLX_OFFLINE=true cargo test --workspace` (must pass)
- Test the specific fix/improvement manually
- Verify no unintended side effects
- Check UI consistency if applicable

### Step 4: Commit & Merge
- Commit with clear description of change
- Merge directly to main (if safe)
- Update relevant documentation if needed
- Create brief memory entry for tracking

---

## ✅ Success Criteria

- **Change scope**: Single file, <50 lines
- **Build status**: Zero compilation errors
- **Test status**: All tests passing
- **Security**: No security implications
- **Brand**: No brand compliance impact
- **Functionality**: No regressions

---

## 🛑 When NOT to Use This Workflow

Use `/feature-development` instead for:
- New features or functionality
- Database schema changes
- Security-related changes
- Brand compliance updates
- Multiple file changes
- Complex logic modifications
- API endpoint changes

---

## 📋 Quick Checklist

**Before Starting:**
- [ ] Change is minor and focused
- [ ] No security implications
- [ ] No brand compliance impact
- [ ] Single file or minimal changes

**Before Merging:**
- [ ] Build passes successfully
- [ ] All tests pass
- [ ] Manual testing confirms fix
- [ ] No regressions detected
- [ ] Documentation updated if needed

---

*This workflow enables rapid iteration for minor changes while maintaining code quality standards.*
