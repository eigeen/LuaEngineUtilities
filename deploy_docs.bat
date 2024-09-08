@echo off

cd docs
mdbook build

cd book
git init
git add .
git config commit.gpgsign false
git commit -m 'deploy'
git branch -M gh-pages
git remote add origin https://github.com/eigeen/LuaEngineUtilities

git push -u -f origin gh-pages