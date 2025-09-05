# Contributing

Thank you for wanting to contribute. This file explains how to report issues, propose changes, and submit pull requests.

## Table of contents
- Reporting issues
- Proposing changes (PRs)
- Development setup
- Tests & linters
- Commit messages & branching
- Code review & CI
- Security
- Translating documentation

## Reporting issues
- Search existing issues first.
- Open a new issue with:
    - Clear title and description
    - Steps to reproduce
    - Expected vs actual behavior
    - Relevant logs / environment (OS, versions)
- For feature requests, explain the problem and proposed solution.

## Proposing changes (pull requests)
- Fork the repository and create a topic branch (feature/bugfix).
- Keep PRs small and focused.
- Update or add tests for new behavior.
- Ensure all tests pass locally and CI is green.
- Include a brief description of changes and rationale.
- Link related issue (e.g., "Closes #123").
This project aims to be language/tool agnostic. Example for Python-based workflows:
- Create virtual env: python -m venv .venv && source .venv/bin/activate
- Install deps: pip install -r requirements.txt
- Run tests: pytest

For other ecosystems, follow the repository README.

## Tests & linters
- Add tests for bugs and new features.
- Run linters/formatters (e.g., black, eslint) before submitting.
- Fix warnings or explain why they are acceptable.

## Commit messages & branching
- Use short, descriptive commits.
- Prefer a branching model like: main ← develop ← feature/*
- Consider Conventional Commits (feat:, fix:, docs:, chore:, etc.).

## Code review & CI
- Maintainters will review and request changes if needed.
- Address review comments in a timely manner.
- Do not merge until CI passes and reviewers approve.

## Security
- Do not disclose security issues in public issues.
- Contact repository maintainers privately to report vulnerabilities.

## Translating documentation
- Contributions to translations are welcome.
- Keep translations consistent and update both languages when modifying docs.

## License & Contributor License Agreement
- Contributions are covered by the project license. By submitting, you agree to license your contribution under the same terms.

---

# Вклад

Спасибо за желание внести вклад. Здесь описано, как сообщать об ошибках, предлагать изменения и отправлять pull request.

## Содержание
- Сообщение об ошибках
- Предложения изменений (PR)
- Настройка окружения
- Тесты и линтеры
- Коммиты и ветвление
- Код-ревью и CI
- Безопасность
- Переводы

## Сообщение об ошибках
- Проверьте открытые issue.
- Создайте новое с:
    - Четким заголовком и описанием
    - Шагами для воспроизведения
    - Ожидаемым и фактическим поведением
    - Логами и окружением (OS, версии)

## Предложения изменений (pull requests)
- Форкните репозиторий и создайте ветку feature/bugfix.
- Держите PR небольшими и целевыми.
- Добавьте/обновите тесты.
- Убедитесь, что тесты проходят локально и CI зелёный.
- Опишите изменения и причину.
- Привяжите связанный issue (например, "Closes #123").

## Тесты и линтеры
- Добавляйте тесты для багов и новых функций.
- Запускайте линтеры/форматтеры (black, eslint) перед отправкой.
- Устраняйте предупреждения или объясняйте их допустимость.

## Коммиты и ветвление
- Короткие и понятные сообщения коммитов.
- Рекомендуемая модель ветвления: main ← develop ← feature/*
- Возможен формат Conventional Commits (feat:, fix:, docs:, и т.д.).

## Код-ревью и CI
- Мейнтейнеры проверяют PR и могут запросить изменения.
- Исправьте замечания в разумные сроки.
- Не сливайте PR до прохождения CI и одобрения.

## Безопасность
- Не публикуйте уязвимости в открытых issue.
- Сообщайте о проблемах безопасности приватно мейнтейнерам репозитория.

## Переводы документации
- Переводы приветствуются.
- Поддерживайте согласованность переводов и обновляйте обе версии при изменениях.

## Лицензия и условия вклада
- Вклады подпадают под лицензию проекта. Отправляя изменения, вы соглашаетесь с тем, что они лицензируются на тех же условиях.

Спасибо за вклад.