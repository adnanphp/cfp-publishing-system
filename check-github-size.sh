#!/bin/bash
# check-github-size.sh - Check what will be pushed to GitHub

echo "📊 Checking GitHub Push Size"
echo "================================"
echo ""

# 1. Check total size of all files (excluding .gitignore)
echo "1️⃣ Total size of ALL files in project:"
du -sh . 2>/dev/null | awk '{print "   📦 Total: " $1}'
echo ""

# 2. Check what files are NOT ignored (will be pushed)
echo "2️⃣ Files that WILL be pushed to GitHub (excluding .gitignore):"
git add -n . 2>/dev/null | wc -l | awk '{print "   📄 Files: " $1}'

# 3. Show size of files that will be pushed
echo "3️⃣ Size of files that will be pushed:"
git ls-files --others --exclude-standard | xargs du -ch 2>/dev/null | tail -1 | awk '{print "   📦 Push size: " $1}'
echo ""

# 4. Show largest files that will be pushed
echo "4️⃣ Top 10 largest files that will be pushed:"
git ls-files --others --exclude-standard | xargs du -h 2>/dev/null | sort -rh | head -10
echo ""

# 5. Show what's in .gitignore
echo "5️⃣ Files ignored by .gitignore:"
echo "   These files will NOT be pushed:"
echo "   - backend/target/ (Rust build files)"
echo "   - frontend/node_modules/ (npm packages)"
echo "   - ml-service/venv/ (Python virtual env)"
echo "   - *.log (log files)"
echo "   - *.db, *.sqlite (database files)"
echo ""

# 6. Check specific directories
echo "6️⃣ Size of key directories:"
if [ -d "backend/target" ]; then
    du -sh backend/target 2>/dev/null | awk '{print "   backend/target/: " $1 " (IGNORED - not pushed)"}'
fi
if [ -d "frontend/node_modules" ]; then
    du -sh frontend/node_modules 2>/dev/null | awk '{print "   frontend/node_modules/: " $1 " (IGNORED - not pushed)"}'
fi
if [ -d "ml-service/venv" ]; then
    du -sh ml-service/venv 2>/dev/null | awk '{print "   ml-service/venv/: " $1 " (IGNORED - not pushed)"}'
fi

# 7. Source code size
echo ""
echo "7️⃣ Source code size (what you're actually pushing):"
find . -name "*.rs" -o -name "*.svelte" -o -name "*.js" -o -name "*.ts" -o -name "*.py" -o -name "*.html" -o -name "*.css" 2>/dev/null | xargs du -ch 2>/dev/null | tail -1 | awk '{print "   📝 Source code: " $1}'
echo ""

# 8. Check for large files that might be accidentally included
echo "8️⃣ Checking for large files (>10MB) not in .gitignore:"
find . -type f -size +10M ! -path "./.git/*" ! -path "./backend/target/*" ! -path "./frontend/node_modules/*" ! -path "./ml-service/venv/*" 2>/dev/null | while read -r file; do
    size=$(du -h "$file" | cut -f1)
    echo "   ⚠️  $file ($size) - ADD TO .gitignore!"
done
echo ""

# 9. Git status
echo "9️⃣ Current git status:"
git status --short 2>/dev/null | head -10
if [ $(git status --short 2>/dev/null | wc -l) -gt 10 ]; then
    echo "   ... and $(($(git status --short 2>/dev/null | wc -l) - 10)) more files"
fi
echo ""

echo "================================"
echo "💡 Recommendations:"
echo ""
echo "If push size is > 100MB, you'll need Git LFS or remove more files."
echo ""
echo "To see exactly what will be pushed:"
echo "  git add -n ."
echo ""
echo "To actually add files:"
echo "  git add ."
echo "  git commit -m \"Initial commit\""
echo "  git push origin main"
