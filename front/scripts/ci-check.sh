#!/usr/bin/env bash
# Локальный скрипт для проверки кода перед коммитом (как в CI/CD)

set -e

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}🔍 Запуск локальных CI проверок${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════${NC}"
echo ""

# Счетчик ошибок
ERRORS=0

# 1. Форматирование
echo -e "${YELLOW}📝 Шаг 1/4: Проверка форматирования...${NC}"
if bun run format:check; then
    echo -e "${GREEN}✅ Форматирование в порядке${NC}"
else
    echo -e "${RED}❌ Проблемы с форматированием. Запустите: bun run format${NC}"
    ((ERRORS++))
fi
echo ""

# 2. Линтинг
echo -e "${YELLOW}🔍 Шаг 2/4: Запуск линтера...${NC}"
if bun run lint; then
    echo -e "${GREEN}✅ Линтинг пройден${NC}"
else
    echo -e "${RED}❌ Ошибки линтинга. Запустите: bun run lint:fix${NC}"
    ((ERRORS++))
fi
echo ""

# 3. Type checking
echo -e "${YELLOW}📝 Шаг 3/4: Проверка типов TypeScript...${NC}"
if bun run type-check; then
    echo -e "${GREEN}✅ Типы корректны${NC}"
else
    echo -e "${RED}❌ Ошибки типизации${NC}"
    ((ERRORS++))
fi
echo ""

# 4. Сборка
echo -e "${YELLOW}🏗️ Шаг 4/4: Сборка проекта...${NC}"
if bun run build; then
    echo -e "${GREEN}✅ Сборка успешна${NC}"
else
    echo -e "${RED}❌ Ошибка сборки${NC}"
    ((ERRORS++))
fi
echo ""

# Итоги
echo -e "${BLUE}════════════════════════════════════════════════════${NC}"
if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}✅ Все проверки пройдены! Готово к коммиту.${NC}"
    echo -e "${BLUE}════════════════════════════════════════════════════${NC}"
    exit 0
else
    echo -e "${RED}❌ Найдено ошибок: $ERRORS${NC}"
    echo -e "${YELLOW}Исправьте ошибки перед коммитом${NC}"
    echo -e "${BLUE}════════════════════════════════════════════════════${NC}"
    exit 1
fi


