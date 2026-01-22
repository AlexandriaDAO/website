import { chromium, Browser, Page, ConsoleMessage } from 'playwright';
import * as fs from 'fs';
import * as path from 'path';

// Configuration
const CONFIG = {
  url: process.env.APP_URL || 'http://localhost:8080',
  outputDir: 'preview_output',
  timeout: 15000,
  waitAfterLoad: 3000,  // Time to let WASM initialize and render
  viewports: [
    { name: 'desktop', width: 1920, height: 1080 },
    { name: 'mobile', width: 390, height: 844 },
    { name: 'tablet', width: 1024, height: 768 },
  ],
};

interface CaptureResult {
  success: boolean;
  screenshots: string[];
  logFile: string;
  errors: string[];
  warnings: string[];
}

async function capture(): Promise<CaptureResult> {
  // Ensure output directory exists
  fs.mkdirSync(CONFIG.outputDir, { recursive: true });

  const logs: string[] = [];
  const errors: string[] = [];
  const warnings: string[] = [];
  const screenshots: string[] = [];
  let success = true;

  const log = (level: string, msg: string) => {
    const line = `[${new Date().toISOString()}] [${level}] ${msg}`;
    logs.push(line);

    if (level === 'ERROR' || level === 'CRASH') {
      errors.push(msg);
      success = false;
    } else if (level === 'WARNING') {
      warnings.push(msg);
    }

    // Also print to terminal for immediate feedback
    console.log(line);
  };

  log('INFO', `Starting capture for ${CONFIG.url}`);

  const browser = await chromium.launch({
    headless: true,
  });

  const context = await browser.newContext();
  const page = await context.newPage();

  // Capture console messages
  page.on('console', (msg: ConsoleMessage) => {
    const type = msg.type().toUpperCase();
    const text = msg.text();

    // Map console types to our log levels
    const level = type === 'ERROR' ? 'ERROR'
                : type === 'WARNING' ? 'WARNING'
                : 'CONSOLE';

    log(level, text);
  });

  // Capture page crashes
  page.on('pageerror', (err: Error) => {
    log('CRASH', `${err.message}\n${err.stack || ''}`);
  });

  // Capture failed network requests
  page.on('requestfailed', (req) => {
    const failure = req.failure();
    log('NETWORK', `Failed to load: ${req.url()} - ${failure?.errorText || 'unknown'}`);
  });

  // Capture WebGL context issues
  page.on('console', (msg) => {
    const text = msg.text();
    if (text.includes('CONTEXT_LOST') || text.includes('WebGL')) {
      log('WEBGL', text);
    }
  });

  // Navigate to page
  try {
    log('INFO', 'Navigating to page...');

    const response = await page.goto(CONFIG.url, {
      timeout: CONFIG.timeout,
      waitUntil: 'domcontentloaded',
    });

    if (response) {
      log('INFO', `Page loaded with status: ${response.status()}`);
    }

    // Wait for WASM to initialize and render
    log('INFO', `Waiting ${CONFIG.waitAfterLoad}ms for WASM initialization...`);
    await page.waitForTimeout(CONFIG.waitAfterLoad);

  } catch (err) {
    log('CRASH', `Navigation failed: ${err}`);
  }

  // Take screenshots at each viewport size
  for (const viewport of CONFIG.viewports) {
    try {
      await page.setViewportSize({ width: viewport.width, height: viewport.height });
      await page.waitForTimeout(500); // Let layout adjust

      const filename = `screenshot_${viewport.name}.png`;
      const filepath = path.join(CONFIG.outputDir, filename);

      await page.screenshot({
        path: filepath,
        fullPage: false,
      });

      screenshots.push(filepath);
      log('INFO', `Screenshot saved: ${filepath} (${viewport.width}x${viewport.height})`);

    } catch (err) {
      log('ERROR', `Screenshot failed for ${viewport.name}: ${err}`);
    }
  }

  // Save all logs
  const logFile = path.join(CONFIG.outputDir, 'console.log');
  fs.writeFileSync(logFile, logs.join('\n'));

  // Create summary JSON
  const summary = {
    timestamp: new Date().toISOString(),
    url: CONFIG.url,
    success,
    errorCount: errors.length,
    warningCount: warnings.length,
    screenshots,
    errors,
    warnings,
  };

  fs.writeFileSync(
    path.join(CONFIG.outputDir, 'summary.json'),
    JSON.stringify(summary, null, 2)
  );

  await browser.close();

  // Print summary
  console.log('\n' + '='.repeat(60));
  console.log('CAPTURE COMPLETE');
  console.log('='.repeat(60));
  console.log(`Status: ${success ? 'SUCCESS' : 'ERRORS DETECTED'}`);
  console.log(`Errors: ${errors.length}`);
  console.log(`Warnings: ${warnings.length}`);
  console.log(`Screenshots: ${screenshots.length}`);
  console.log(`Log file: ${logFile}`);
  console.log('='.repeat(60) + '\n');

  return { success, screenshots, logFile, errors, warnings };
}

// Run
capture().then((result) => {
  process.exit(result.success ? 0 : 1);
}).catch((err) => {
  console.error('Capture script failed:', err);
  process.exit(1);
});
