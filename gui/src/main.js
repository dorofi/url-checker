// Import Tauri API
let invoke = null;
let tauriInitialized = false;

// Initialize Tauri API
async function initTauri() {
    if (tauriInitialized) {
        return invoke !== null;
    }
    
    try {
        // Try to import Tauri API
        const tauriApi = await import('@tauri-apps/api/tauri');
        invoke = tauriApi.invoke;
        tauriInitialized = true;
        
        // Verify it actually works by checking if we're in Tauri context
        // In Tauri, window object has specific properties
        const isInTauri = typeof window !== 'undefined' && 
                         (window.__TAURI_IPC__ || 
                          window.__TAURI_METADATA__ || 
                          window.__TAURI_INTERNALS__ ||
                          window.__TAURI_CHECK__ ||
                          (navigator.userAgent && navigator.userAgent.includes('Tauri')));
        
        if (isInTauri) {
            console.log('Tauri API loaded successfully');
            return true;
        } else {
            console.warn('Tauri API imported but not running in Tauri context');
            invoke = null;
            return false;
        }
    } catch (e) {
        console.error('Failed to load Tauri API:', e);
        invoke = null;
        tauriInitialized = true; // Mark as initialized to avoid retrying
        return false;
    }
}

// Try to initialize immediately
initTauri();

// DOM elements
const urlsInput = document.getElementById('urls-input');
const concurrencyInput = document.getElementById('concurrency');
const timeoutInput = document.getElementById('timeout');
const checkBtn = document.getElementById('check-btn');
const statsContainer = document.getElementById('stats');
const resultsTable = document.getElementById('results-table');

// Format file size helper
function formatSize(bytes) {
    if (bytes === 0) return 'N/A';
    const units = ['B', 'KB', 'MB', 'GB'];
    let size = bytes;
    let unitIndex = 0;
    while (size >= 1024 && unitIndex < units.length - 1) {
        size /= 1024;
        unitIndex++;
    }
    return unitIndex === 0 
        ? `${bytes} ${units[unitIndex]}`
        : `${size.toFixed(2)} ${units[unitIndex]}`;
}

// Format time helper
function formatTime(ms) {
    if (ms === 0) return 'N/A';
    return `${ms} ms`;
}

// Get status badge class
function getStatusClass(status) {
    if (status.startsWith('2')) return 'success';
    if (status.startsWith('3')) return 'redirect';
    return 'error';
}

// Display statistics
function displayStats(stats) {
    const successRate = stats.total > 0 
        ? ((stats.up / stats.total) * 100).toFixed(1) 
        : 0;

    statsContainer.innerHTML = `
        <div class="stat-card">
            <div class="value">${stats.total}</div>
            <div class="label">Total URLs</div>
        </div>
        <div class="stat-card success">
            <div class="value">${stats.up}</div>
            <div class="label">Successful (${successRate}%)</div>
        </div>
        <div class="stat-card error">
            <div class="value">${stats.down}</div>
            <div class="label">Failed</div>
        </div>
        <div class="stat-card">
            <div class="value">${formatTime(stats.avg_time)}</div>
            <div class="label">Avg Response Time</div>
        </div>
        <div class="stat-card">
            <div class="value">${formatTime(stats.min_time)}</div>
            <div class="label">Fastest</div>
        </div>
        <div class="stat-card">
            <div class="value">${formatTime(stats.max_time)}</div>
            <div class="label">Slowest</div>
        </div>
    `;
}

// Display results table
function displayResults(results) {
    if (results.length === 0) {
        resultsTable.innerHTML = '<p class="loading">No results to display</p>';
        return;
    }

    let tableHTML = `
        <table>
            <thead>
                <tr>
                    <th>URL</th>
                    <th>Status</th>
                    <th>Time</th>
                    <th>Size</th>
                    <th>Timestamp</th>
                </tr>
            </thead>
            <tbody>
    `;

    results.forEach(result => {
        const statusClass = getStatusClass(result.status);
        tableHTML += `
            <tr>
                <td>${result.url}</td>
                <td>
                    <span class="status-badge ${statusClass}">
                        ${result.status} ${result.reason}
                    </span>
                </td>
                <td>${formatTime(result.time_ms)}</td>
                <td>${formatSize(result.size_bytes)}</td>
                <td>${result.timestamp}</td>
            </tr>
        `;
    });

    tableHTML += `
            </tbody>
        </table>
    `;

    resultsTable.innerHTML = tableHTML;
}

// Handle check button click
checkBtn.addEventListener('click', async () => {
    const urlsText = urlsInput.value.trim();
    if (!urlsText) {
        alert('Please enter at least one URL');
        return;
    }

    const urls = urlsText
        .split('\n')
        .map(line => line.trim())
        .filter(line => line.length > 0);

    if (urls.length === 0) {
        alert('Please enter at least one valid URL');
        return;
    }

    // Disable button and show loading
    checkBtn.disabled = true;
    checkBtn.textContent = 'Checking...';
    statsContainer.innerHTML = '<div class="loading"><div class="spinner"></div><p>Checking URLs...</p></div>';
    resultsTable.innerHTML = '';

    try {
        // Ensure Tauri is initialized and available
        const tauriReady = await initTauri();
        if (!tauriReady || !invoke) {
            throw new Error('Tauri is not available. Make sure you run: npm run tauri:dev');
        }

        const request = {
            urls: urls,
            timeout: parseInt(timeoutInput.value) || 10,
            concurrency: parseInt(concurrencyInput.value) || 20,
        };

        const response = await invoke('check_urls', { request });
        
        // Display results
        displayStats(response.stats);
        displayResults(response.results);
    } catch (error) {
        console.error('Error:', error);
        const errorMsg = error.message || String(error);
        alert(`Error: ${errorMsg}`);
        statsContainer.innerHTML = '';
        resultsTable.innerHTML = `<p class="loading" style="color: #dc3545; text-align: center;">
            <strong>Error occurred:</strong><br>
            ${errorMsg}<br><br>
            <strong>To run this application:</strong><br>
            <code style="background: #f0f0f0; padding: 5px; border-radius: 3px; display: inline-block; margin-top: 10px;">cd gui && npm run tauri:dev</code>
        </p>`;
    } finally {
        checkBtn.disabled = false;
        checkBtn.textContent = 'Check URLs';
    }
});

// Allow Enter key to trigger check (Ctrl+Enter)
urlsInput.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === 'Enter') {
        checkBtn.click();
    }
});

