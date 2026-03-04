# Git Commit Enforcement (Repository-Specific)

## Scope
ONLY applies to current repository

## Purpose
Ensure all code changes in this repository are properly committed when requested.

## Activation Triggers
- User explicitly mentions "commit" or "commit changes"
- Code files are modified (src/, examples/, Cargo.toml, etc.)
- User asks to "save changes" or similar
- Any code modification is completed

## Required Actions
1. **Check Status First**: Always run `git status` to show what changed
2. **Show Changes**: Display `git diff --stat` summary
3. **Stage Changes**: Use `git add` for all modified files
4. **Suggest Message**: Propose commit message based on changes made
5. **Execute Commit**: Run `git commit -m "message"`
6. **Verify Success**: Show commit hash and confirm working tree clean
7. **Report**: Always tell user "✅ Commit successful: [hash]"

## Safety Rules
- Never commit without explicit user confirmation
- Always show what will be committed before committing
- Verify working tree is clean after commit
- If no changes: Inform user "No changes to commit"
- If commit fails: Show error and suggest fixes

## Example Workflow
User: "Fix the library API"
→ Make code changes
→ Show: "📝 Changes detected in src/api/libraries.rs"
→ Show: `git diff --stat`
→ Ask: "Commit these changes? [y/N]"
→ If yes: Stage → Commit → Verify → Confirm success

## Commit Message Format
Use conventional commits style:
- feat: for new features
- fix: for bug fixes  
- docs: for documentation changes
- refactor: for code improvements
- chore: for maintenance tasks

## Verification
After every commit:
- Show commit hash
- Show commit message
- Confirm "Working tree clean"
- Show git status
