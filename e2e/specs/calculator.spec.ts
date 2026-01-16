import { test, expect } from '@playwright/test';
import { CalculatorPage, WindowManagerPage, DesktopPage } from '../page-objects';

test.describe('Calculator', () => {
  let calculator: CalculatorPage;
  let windowManager: WindowManagerPage;

  test.beforeEach(async ({ page }) => {
    const desktop = new DesktopPage(page);
    await desktop.goto();
    windowManager = new WindowManagerPage(page);
    const calcWindow = windowManager.getWindow('Calculator');
    // Use JavaScript to bring the Calculator window to the front by clicking it
    await calcWindow.evaluate((el) => {
      // Dispatch mousedown event to trigger bring_to_front
      el.dispatchEvent(new MouseEvent('mousedown', { bubbles: true }));
    });
    calculator = new CalculatorPage(page, calcWindow);
  });

  test.describe('Display', () => {
    test('should show 0 by default', async () => {
      const value = await calculator.getDisplayValue();
      expect(value).toBe('0');
    });

    test('should display entered digits', async () => {
      await calculator.pressDigit('5');
      expect(await calculator.getDisplayValue()).toBe('5');

      await calculator.pressDigit('7');
      expect(await calculator.getDisplayValue()).toBe('57');
    });

    test('should replace leading zero with digit', async () => {
      await calculator.pressDigit('3');
      expect(await calculator.getDisplayValue()).toBe('3');
    });
  });

  test.describe('Addition', () => {
    test('should add two single-digit numbers: 5 + 3 = 8', async () => {
      await calculator.calculate('5+3');
      expect(await calculator.getDisplayValue()).toBe('8');
    });

    test('should add two-digit numbers: 25 + 17 = 42', async () => {
      await calculator.pressClear();
      await calculator.pressDigit('2');
      await calculator.pressDigit('5');
      await calculator.pressOperator('+');
      await calculator.pressDigit('1');
      await calculator.pressDigit('7');
      await calculator.pressEquals();

      expect(await calculator.getDisplayValue()).toBe('42');
    });

    test('should add decimal numbers: 1.5 + 2.5 = 4', async () => {
      await calculator.pressClear();
      await calculator.pressDigit('1');
      await calculator.pressDecimal();
      await calculator.pressDigit('5');
      await calculator.pressOperator('+');
      await calculator.pressDigit('2');
      await calculator.pressDecimal();
      await calculator.pressDigit('5');
      await calculator.pressEquals();

      expect(await calculator.getDisplayValue()).toBe('4');
    });
  });

  test.describe('Subtraction', () => {
    test('should subtract: 9 - 4 = 5', async () => {
      await calculator.calculate('9-4');
      expect(await calculator.getDisplayValue()).toBe('5');
    });

    test('should handle negative result: 4 - 9 = -5', async () => {
      await calculator.calculate('4-9');
      expect(await calculator.getDisplayValue()).toBe('-5');
    });

    test('should subtract decimals: 5.5 - 2.2 = 3.3', async () => {
      await calculator.pressClear();
      await calculator.pressDigit('5');
      await calculator.pressDecimal();
      await calculator.pressDigit('5');
      await calculator.pressOperator('−');
      await calculator.pressDigit('2');
      await calculator.pressDecimal();
      await calculator.pressDigit('2');
      await calculator.pressEquals();

      const result = await calculator.getDisplayValue();
      expect(parseFloat(result)).toBeCloseTo(3.3, 1);
    });
  });

  test.describe('Multiplication', () => {
    test('should multiply: 6 × 7 = 42', async () => {
      await calculator.calculate('6×7');
      expect(await calculator.getDisplayValue()).toBe('42');
    });

    test('should multiply by zero: 5 × 0 = 0', async () => {
      await calculator.calculate('5×0');
      expect(await calculator.getDisplayValue()).toBe('0');
    });
  });

  test.describe('Division', () => {
    test('should divide: 8 ÷ 2 = 4', async () => {
      await calculator.calculate('8÷2');
      expect(await calculator.getDisplayValue()).toBe('4');
    });

    test('should handle divide by zero', async () => {
      await calculator.calculate('5÷0');
      const result = await calculator.getDisplayValue();
      expect(result === 'Error' || result === 'Infinity' || result === '∞').toBe(true);
    });
  });

  test.describe('AC (All Clear)', () => {
    test('should clear display to 0', async () => {
      await calculator.pressDigit('1');
      await calculator.pressDigit('2');
      await calculator.pressDigit('3');

      await calculator.pressClear();

      expect(await calculator.getDisplayValue()).toBe('0');
    });

    test('should clear pending operation', async () => {
      await calculator.pressDigit('5');
      await calculator.pressOperator('+');
      await calculator.pressDigit('3');

      await calculator.pressClear();
      await calculator.pressDigit('7');
      await calculator.pressEquals();

      expect(await calculator.getDisplayValue()).toBe('7');
    });
  });

  test.describe('+/− (Negate)', () => {
    test('should negate positive number', async () => {
      await calculator.pressDigit('5');
      await calculator.pressNegate();

      expect(await calculator.getDisplayValue()).toBe('-5');
    });

    test('should negate negative number back to positive', async () => {
      await calculator.pressDigit('5');
      await calculator.pressNegate();
      await calculator.pressNegate();

      expect(await calculator.getDisplayValue()).toBe('5');
    });
  });

  test.describe('% (Percent)', () => {
    test('should convert to percentage: 50 -> 0.5', async () => {
      await calculator.pressDigit('5');
      await calculator.pressDigit('0');
      await calculator.pressPercent();

      expect(await calculator.getDisplayValue()).toBe('0.5');
    });

    test('should convert: 100 -> 1', async () => {
      await calculator.pressDigit('1');
      await calculator.pressDigit('0');
      await calculator.pressDigit('0');
      await calculator.pressPercent();

      expect(await calculator.getDisplayValue()).toBe('1');
    });
  });

  test.describe('Decimal Point', () => {
    test('should allow decimal starting with 0', async () => {
      await calculator.pressClear();
      await calculator.pressDecimal();
      await calculator.pressDigit('5');

      expect(await calculator.getDisplayValue()).toBe('0.5');
    });
  });

  test.describe('Chained Operations', () => {
    test('should chain operations: 5 + 3 + 2 = 10', async () => {
      await calculator.pressClear();
      await calculator.pressDigit('5');
      await calculator.pressOperator('+');
      await calculator.pressDigit('3');
      await calculator.pressOperator('+');
      await calculator.pressDigit('2');
      await calculator.pressEquals();

      expect(await calculator.getDisplayValue()).toBe('10');
    });
  });

});
