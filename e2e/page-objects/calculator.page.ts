import { Page, Locator } from '@playwright/test';

export class CalculatorPage {
  readonly page: Page;
  readonly calculator: Locator;
  readonly display: Locator;
  readonly displayText: Locator;
  readonly buttons: Locator;

  constructor(page: Page, windowLocator: Locator) {
    this.page = page;
    this.calculator = windowLocator.locator('.calculator');
    this.display = this.calculator.locator('.calc-display');
    this.displayText = this.calculator.locator('.calc-display-text');
    this.buttons = this.calculator.locator('.calc-buttons');
  }

  getButton(label: string): Locator {
    return this.buttons.locator('.calc-btn', { hasText: label }).first();
  }

  getDigitButton(digit: string): Locator {
    const escapedDigit = digit.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    return this.buttons.locator('.calc-btn.digit', { hasText: new RegExp(`^${escapedDigit}$`) }).first();
  }

  getOperatorButton(op: string): Locator {
    const escapedOp = op.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    return this.buttons.locator('.calc-btn.operator', { hasText: new RegExp(`^${escapedOp}$`) }).first();
  }

  getFunctionButton(fn: string): Locator {
    return this.buttons.locator('.calc-btn.function', { hasText: fn }).first();
  }

  async pressDigit(digit: string) {
    await this.getDigitButton(digit).click();
  }

  async pressOperator(op: string) {
    await this.getOperatorButton(op).click();
  }

  async pressFunction(fn: string) {
    await this.getFunctionButton(fn).click();
  }

  async pressEquals() {
    await this.getOperatorButton('=').click();
  }

  async pressClear() {
    await this.getFunctionButton('AC').click();
  }

  async pressNegate() {
    await this.getFunctionButton('+/−').click();
  }

  async pressPercent() {
    await this.getFunctionButton('%').click();
  }

  async pressDecimal() {
    await this.getDigitButton('.').click();
  }

  async getDisplayValue(): Promise<string> {
    const text = await this.displayText.textContent();
    return text || '';
  }

  async calculate(expression: string) {
    const match = expression.match(/^(\d+(?:\.\d+)?)([\+\-\*\/×÷])(\d+(?:\.\d+)?)$/);
    if (!match) throw new Error(`Invalid expression: ${expression}`);

    const [, num1, op, num2] = match;

    await this.pressClear();

    for (const digit of num1) {
      if (digit === '.') {
        await this.pressDecimal();
      } else {
        await this.pressDigit(digit);
      }
    }

    const opMap: Record<string, string> = {
      '+': '+',
      '-': '−',
      '*': '×',
      '/': '÷',
      '×': '×',
      '÷': '÷'
    };
    await this.pressOperator(opMap[op] || op);

    for (const digit of num2) {
      if (digit === '.') {
        await this.pressDecimal();
      } else {
        await this.pressDigit(digit);
      }
    }

    await this.pressEquals();
  }
}
