# Behavior / process

- Recognize if the prompt is a simple question or a task to execute
- If in doubt on the task, always ask questions before starting development

# Environment / setup

- Always use .venv (not venv)
- Upgrade pip in virtual environment
- Remove version restrictions from requirements.txt if possible
- Try to use latest versions of packages

# Project structure

- Keep project structure clean
- Remove unnecessary files
- No documentation files (.md files)
- If a Python file is empty, delete it if safe to delete (some are created by Django like tests.py)

# Code style (general)

- Before saving changes to .py files, always run the black formatter with max-line-length=79
- Remove all comments and docstrings
- Keep code minimal and clean
- Never use print statements unless explicitly requested
- Always import the most specific object possible
  - Example: `from django.db.models import CharField` instead of `from django.db import models` then `models.CharField`
  - Example: `from django.contrib.admin import register` instead of `from django.contrib import admin` then `admin.register`

# Python / type hints

- Type hints are not required in this project

# Django models / database

- Model names: Singular (e.g., Disegno, Annotazione)
- Table names: Plural (e.g., disegni, annotazioni)
- Always explicitly declare primary key field: id = models.AutoField(primary_key=True)
- In models.py always set Verbose Name for every field (except id) and use verbose name in all the views and forms
- Store files in database in their own table (id, file)
- Other tables reference files by ID (ForeignKey) not by filename
- Use BinaryField for file data storage
- Avoid filesystem storage to prevent performance issues

# Django migrations

- NEVER modify migration files manually. Always use `python manage.py makemigrations` to create new migrations. Migration files should only be created by Django's makemigrations command, never edited by hand.

# Commits

- Create commits only when explicitly asked
- Write concise commit messages focused on *why*, not *what*
- Never force-push or bypass hooks

# Minified files

- If a file is minified it should stay minified after modifications
