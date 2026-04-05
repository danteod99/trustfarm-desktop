/**
 * 简化版分发商包构建脚本
 * 策略: 在构建前注入分发商标识,为每个分发商单独构建完整的 MSI
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { execSync } from 'child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 配置
const CONFIG = {
    distributorsFile: path.join(__dirname, '..', 'distributors.json'),
    distributorIdFile: path.join(__dirname, '..', 'src-tauri', 'distributor.txt'),
    buildScript: path.join(__dirname, '..', 'build.ps1'),
    msiDir: path.join(__dirname, '..', 'src-tauri', 'target', 'release', 'bundle', 'msi'),
    outputDir: path.join(__dirname, '..', 'dist-distributors'),
};

/**
 * 读取分发商列表
 */
function loadDistributors() {
    if (!fs.existsSync(CONFIG.distributorsFile)) {
        throw new Error(`Distributors file not found: ${CONFIG.distributorsFile}`);
    }

    const content = fs.readFileSync(CONFIG.distributorsFile, 'utf-8');
    const data = JSON.parse(content);

    return data.distributors.filter(d => d.enabled);
}

/**
 * 设置分发商标识
 */
function setDistributorId(code) {
    console.log(`💉 Setting distributor ID: ${code}`);

    // 确保目录存在
    const dir = path.dirname(CONFIG.distributorIdFile);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }

    // 写入分发商代码
    fs.writeFileSync(CONFIG.distributorIdFile, code, 'utf-8');
    console.log(`✅ Distributor ID file created: ${CONFIG.distributorIdFile}`);

    // 同时更新 tauri.conf.json 的 resources
    updateTauriConfig(true);
}

/**
 * 清除分发商标识
 */
function clearDistributorId() {
    if (fs.existsSync(CONFIG.distributorIdFile)) {
        fs.unlinkSync(CONFIG.distributorIdFile);
        console.log('🗑️  Distributor ID file removed');
    }

    // 恢复 tauri.conf.json 的 resources
    updateTauriConfig(false);
}

/**
 * 更新 tauri.conf.json 的 resources 配置
 */
function updateTauriConfig(includeDistributor) {
    const tauriConfigPath = path.join(__dirname, '..', 'src-tauri', 'tauri.conf.json');
    const config = JSON.parse(fs.readFileSync(tauriConfigPath, 'utf-8'));

    const resources = config.tauri.bundle.resources || [];
    const distributorFile = 'distributor.txt';

    // 移除旧的 distributor.txt (如果存在)
    const filteredResources = resources.filter(r => r !== distributorFile);

    // 如果需要,添加 distributor.txt
    if (includeDistributor) {
        filteredResources.push(distributorFile);
    }

    config.tauri.bundle.resources = filteredResources;
    fs.writeFileSync(tauriConfigPath, JSON.stringify(config, null, 2), 'utf-8');
}

/**
 * 执行构建
 */
function runBuild() {
    console.log('🔨 Running build...');

    try {
        // 在 Windows 上运行 PowerShell 脚本
        execSync(`powershell -ExecutionPolicy Bypass -File "${CONFIG.buildScript}"`, {
            stdio: 'inherit',
            cwd: path.dirname(CONFIG.buildScript)
        });

        console.log('✅ Build completed successfully');
    } catch (error) {
        throw new Error(`Build failed: ${error.message}`);
    }
}

/**
 * 查找并移动生成的 MSI
 */
function moveBuiltMsi(distributorCode) {
    console.log('📦 Looking for built MSI...');

    if (!fs.existsSync(CONFIG.msiDir)) {
        throw new Error(`MSI directory not found: ${CONFIG.msiDir}`);
    }

    const files = fs.readdirSync(CONFIG.msiDir);
    const msiFiles = files.filter(f => f.endsWith('.msi'));

    if (msiFiles.length === 0) {
        throw new Error(`No MSI file found in ${CONFIG.msiDir}`);
    }

    const sourceMsi = path.join(CONFIG.msiDir, msiFiles[0]);

    // 生成目标文件名
    const baseName = path.basename(msiFiles[0], '.msi');
    const targetName = distributorCode === 'OFFICIAL'
        ? `${baseName}.msi`
        : `${baseName}_${distributorCode}.msi`;

    const targetMsi = path.join(CONFIG.outputDir, targetName);

    // 确保输出目录存在
    if (!fs.existsSync(CONFIG.outputDir)) {
        fs.mkdirSync(CONFIG.outputDir, { recursive: true });
    }

    // 移动文件
    fs.copyFileSync(sourceMsi, targetMsi);
    console.log(`✅ MSI saved: ${targetName}`);

    return targetMsi;
}

/**
 * 主函数
 */
async function main() {
    console.log('🚀 Starting distributor package build (Simple Strategy)...\n');

    try {
        // 清理旧的输出: msiDir, outputDir
        if (fs.existsSync(CONFIG.msiDir)) {
            fs.rmSync(CONFIG.msiDir, { recursive: true, force: true });
            console.log(`🗑️  Old MSI directory cleared: ${CONFIG.msiDir}`);
        }
        if (fs.existsSync(CONFIG.outputDir)) {
            fs.rmSync(CONFIG.outputDir, { recursive: true, force: true });
            console.log(`🗑️  Old output directory cleared: ${CONFIG.outputDir}`);
        }
        // 1. 读取分发商列表
        const distributors = loadDistributors();
        console.log(`📋 Found ${distributors.length} enabled distributors:\n`);
        distributors.forEach(d => {
            console.log(`   - ${d.code}: ${d.name}`);
        });
        console.log('');

        // 2. 为每个分发商构建
        const builtPackages = [];

        for (let i = 0; i < distributors.length; i++) {
            const distributor = distributors[i];

            console.log(`\n${'='.repeat(70)}`);
            console.log(`[${i + 1}/${distributors.length}] Building for: ${distributor.code} - ${distributor.name}`);
            console.log('='.repeat(70));

            try {
                // 2.1 设置分发商标识
                if (distributor.code === 'OFFICIAL') {
                    clearDistributorId();
                } else {
                    setDistributorId(distributor.code);
                }

                // 2.2 执行构建
                runBuild();

                // 2.3 移动生成的 MSI
                const msiPath = moveBuiltMsi(distributor.code);
                builtPackages.push({
                    distributor: distributor.code,
                    file: path.basename(msiPath),
                    path: msiPath
                });

                console.log(`✅ Package for ${distributor.code} completed!`);

            } catch (error) {
                console.error(`❌ Failed to build for ${distributor.code}: ${error.message}`);
                // 继续处理下一个分发商
            }
        }

        // 3. 清理
        clearDistributorId();

        // 4. 总结
        console.log(`\n${'='.repeat(70)}`);
        console.log('✅ All distributor packages built successfully!');
        console.log('='.repeat(70));
        console.log('\n📦 Built packages:');
        builtPackages.forEach(pkg => {
            console.log(`   - ${pkg.distributor}: ${pkg.file}`);
        });
        console.log(`\n📂 Output directory: ${CONFIG.outputDir}`);
        console.log('='.repeat(70));

    } catch (error) {
        console.error(`\n❌ Error: ${error.message}`);
        clearDistributorId(); // 确保清理
        process.exit(1);
    }
}

main();
