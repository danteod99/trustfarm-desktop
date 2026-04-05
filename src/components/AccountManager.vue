<template>
  <dialog ref="dialog" class="modal">
    <div class="modal-box max-w-4xl">
      <div class="flex items-center justify-between mb-4">
        <h3 class="font-bold text-lg">Account Manager</h3>
        <button class="btn btn-sm btn-ghost" @click="$refs.dialog.close()">
          <font-awesome-icon icon="fa-solid fa-xmark" class="h-4 w-4" />
        </button>
      </div>

      <!-- Platform selector -->
      <div class="mb-4">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1">Platform</div>
        <div class="flex gap-1">
          <button v-for="p in platforms" :key="p.id" class="btn btn-sm flex-1"
            :class="selectedPlatform === p.id ? 'btn-primary' : 'btn-outline'"
            @click="selectedPlatform = p.id">
            <font-awesome-icon :icon="p.icon" class="h-3 w-3" /> {{ p.label }}
          </button>
        </div>
      </div>

      <!-- Import section -->
      <div class="mb-4">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1">Import Accounts</div>
        <div class="text-xs text-base-content/50 mb-2">
          Paste accounts in any format. Auto-detects: user:pass, user:pass:2fa, email:pass, user:pass:email:emailpass:2fa
          <br>Separators: <code>:</code> <code>|</code> <code>tab</code>
        </div>
        <textarea class="textarea textarea-bordered w-full h-28 text-sm font-mono" v-model="importText"
          placeholder="user1:password1:2fa_code&#10;user2:password2&#10;email@mail.com:password3:user3:emailpass:2fa_code"></textarea>
        <div class="flex gap-2 mt-2">
          <button class="btn btn-sm btn-primary" @click="parseAccounts">
            <font-awesome-icon icon="fa-solid fa-file-import" class="h-3 w-3" /> Import
          </button>
          <button class="btn btn-sm btn-outline" @click="importText = ''">Clear</button>
          <span class="text-xs text-base-content/50 self-center" v-if="parseMessage">{{ parseMessage }}</span>
        </div>
      </div>

      <div class="divider my-2"></div>

      <!-- Accounts table -->
      <div class="mb-4">
        <div class="flex items-center justify-between mb-2">
          <div class="text-xs font-bold text-base-content/50 uppercase">
            Accounts ({{ accounts.length }})
          </div>
          <div class="flex gap-1">
            <button class="btn btn-sm btn-success" @click="loginAll" :disabled="accounts.length === 0">
              <font-awesome-icon icon="fa-solid fa-right-to-bracket" class="h-3 w-3" /> Login All
            </button>
            <button class="btn btn-sm btn-error btn-outline" @click="clearAll" :disabled="accounts.length === 0">
              <font-awesome-icon icon="fa-solid fa-trash" class="h-3 w-3" /> Clear All
            </button>
          </div>
        </div>

        <div class="overflow-x-auto max-h-72 overflow-y-auto">
          <table class="table table-sm table-zebra w-full" v-if="accounts.length > 0">
            <thead>
              <tr>
                <th class="w-8">#</th>
                <th>Platform</th>
                <th>Username</th>
                <th>Password</th>
                <th>2FA</th>
                <th>Email</th>
                <th>Email Pass</th>
                <th>Device</th>
                <th>Status</th>
                <th class="w-8"></th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(acc, idx) in accounts" :key="idx">
                <td class="text-xs">{{ idx + 1 }}</td>
                <td>
                  <font-awesome-icon :icon="platformIcon(acc.platform)" class="h-3 w-3" :class="platformColor(acc.platform)" />
                </td>
                <td>
                  <input class="input input-xs input-bordered w-full" v-model="acc.username" />
                </td>
                <td>
                  <input class="input input-xs input-bordered w-full" v-model="acc.password" type="password" />
                </td>
                <td>
                  <input class="input input-xs input-bordered w-20" v-model="acc.twofa" />
                </td>
                <td>
                  <input class="input input-xs input-bordered w-full" v-model="acc.email" />
                </td>
                <td>
                  <input class="input input-xs input-bordered w-full" v-model="acc.emailPass" type="password" />
                </td>
                <td>
                  <span class="text-xs font-mono" :class="acc.device ? 'text-success' : 'text-base-content/40'">
                    {{ acc.device ? (acc.deviceIndex || acc.device.substring(0,8) + '...') : 'Unassigned' }}
                  </span>
                </td>
                <td>
                  <span class="badge badge-sm" :class="statusClass(acc.status)">{{ acc.status || 'idle' }}</span>
                </td>
                <td>
                  <button class="btn btn-xs btn-ghost text-error" @click="removeAccount(idx)">
                    <font-awesome-icon icon="fa-solid fa-xmark" class="h-3 w-3" />
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
          <div v-else class="text-center text-sm text-base-content/50 py-8">
            No accounts added. Paste accounts above to import.
          </div>
        </div>
      </div>

      <div class="divider my-2"></div>

      <!-- Manual add -->
      <div class="mb-2">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1">Add Single Account</div>
        <div class="flex gap-1 flex-wrap">
          <input class="input input-sm input-bordered w-28" v-model="manual.username" placeholder="Username" />
          <input class="input input-sm input-bordered w-28" v-model="manual.password" placeholder="Password" type="password" />
          <input class="input input-sm input-bordered w-20" v-model="manual.twofa" placeholder="2FA" />
          <input class="input input-sm input-bordered w-36" v-model="manual.email" placeholder="Email" />
          <input class="input input-sm input-bordered w-28" v-model="manual.emailPass" placeholder="Email Pass" type="password" />
          <button class="btn btn-sm btn-primary" @click="addManual">
            <font-awesome-icon icon="fa-solid fa-plus" class="h-3 w-3" />
          </button>
        </div>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop"><button>close</button></form>
  </dialog>
</template>

<script>
import { getItem, setItem } from '@/utils/storage.js';

export default {
  name: 'AccountManager',
  props: ['devices'],
  data() {
    return {
      accounts: [],
      importText: '',
      parseMessage: '',
      selectedPlatform: 'instagram',
      platforms: [
        { id: 'instagram', label: 'Instagram', icon: 'fa-brands fa-instagram', package: 'com.instagram.android', activity: 'com.instagram.android/com.instagram.nux.activity.SignedOutFragmentActivity' },
        { id: 'facebook', label: 'Facebook', icon: 'fa-brands fa-facebook', package: 'com.facebook.katana', activity: 'com.facebook.katana/com.facebook.katana.LoginActivity' },
        { id: 'tiktok', label: 'TikTok', icon: 'fa-brands fa-tiktok', package: 'com.zhiliaoapp.musically', activity: 'com.zhiliaoapp.musically/com.ss.android.ugc.aweme.splash.SplashActivity' },
      ],
      manual: { username: '', password: '', twofa: '', email: '', emailPass: '' },
    }
  },
  methods: {
    show() {
      this.$refs.dialog.showModal();
    },
    platformIcon(platform) {
      const p = this.platforms.find(x => x.id === platform);
      return p ? p.icon : 'fa-solid fa-question';
    },
    platformColor(platform) {
      switch(platform) {
        case 'instagram': return 'text-pink-500';
        case 'facebook': return 'text-blue-600';
        case 'tiktok': return 'text-black';
        default: return '';
      }
    },
    parseAccounts() {
      const lines = this.importText.trim().split('\n').filter(l => l.trim());
      let added = 0;

      for (const line of lines) {
        // Split by : | or tab
        const parts = line.trim().split(/[:||\t]+/).map(p => p.trim()).filter(p => p);
        if (parts.length < 2) continue;

        const acc = { username: '', password: '', twofa: '', email: '', emailPass: '', device: '', deviceIndex: '', status: 'idle', platform: this.selectedPlatform };

        // Detect format based on number of parts and content
        const emailIdx = parts.findIndex(p => p.includes('@'));

        if (emailIdx >= 0) {
          // Has email
          if (parts.length === 2) {
            // email:pass
            acc.email = parts[0];
            acc.password = parts[1];
            acc.username = parts[0].split('@')[0];
          } else if (parts.length === 3) {
            if (emailIdx === 0) {
              // email:pass:2fa or email:pass:username
              acc.email = parts[0];
              acc.password = parts[1];
              if (parts[2].length <= 8 && /^\d+$/.test(parts[2])) {
                acc.twofa = parts[2];
                acc.username = parts[0].split('@')[0];
              } else {
                acc.username = parts[2];
              }
            } else if (emailIdx === 2) {
              // user:pass:email
              acc.username = parts[0];
              acc.password = parts[1];
              acc.email = parts[2];
            } else {
              // user:email:pass
              acc.username = parts[0];
              acc.email = parts[1];
              acc.password = parts[2];
            }
          } else if (parts.length === 4) {
            // user:pass:email:emailpass or user:pass:email:2fa
            acc.username = parts[0];
            acc.password = parts[1];
            acc.email = parts[emailIdx];
            const remaining = parts.filter((_, i) => i !== 0 && i !== 1 && i !== emailIdx)[0];
            if (remaining && remaining.length <= 8 && /^\d+$/.test(remaining)) {
              acc.twofa = remaining;
            } else {
              acc.emailPass = remaining || '';
            }
          } else if (parts.length >= 5) {
            // user:pass:email:emailpass:2fa
            acc.username = parts[0];
            acc.password = parts[1];
            acc.email = parts[emailIdx];
            // Find remaining parts
            const used = new Set([0, 1, emailIdx]);
            const rest = parts.filter((_, i) => !used.has(i));
            if (rest.length >= 2) {
              acc.emailPass = rest[0];
              acc.twofa = rest[1];
            } else if (rest.length === 1) {
              if (rest[0].length <= 8 && /^\d+$/.test(rest[0])) {
                acc.twofa = rest[0];
              } else {
                acc.emailPass = rest[0];
              }
            }
          }
        } else {
          // No email
          acc.username = parts[0];
          acc.password = parts[1];
          if (parts.length >= 3) acc.twofa = parts[2];
          if (parts.length >= 4) acc.email = parts[3];
          if (parts.length >= 5) acc.emailPass = parts[4];
        }

        if (acc.username || acc.email) {
          this.accounts.push(acc);
          added++;
        }
      }

      this.parseMessage = `Imported ${added} accounts`;
      this.importText = '';
      this.saveAccounts();
    },
    addManual() {
      if (!this.manual.username && !this.manual.email) return;
      this.accounts.push({
        ...this.manual,
        platform: this.selectedPlatform,
        device: '',
        deviceIndex: '',
        status: 'idle',
      });
      this.manual = { username: '', password: '', twofa: '', email: '', emailPass: '' };
      this.saveAccounts();
    },
    removeAccount(idx) {
      this.accounts.splice(idx, 1);
      this.saveAccounts();
    },
    clearAll() {
      this.accounts = [];
      this.saveAccounts();
    },
    async loginAll() {
      if (!this.devices || this.devices.length === 0) {
        await this.$emiter('NOTIFY', { type: 'error', message: 'No devices connected', timeout: 2000 });
        return;
      }

      const unassigned = this.accounts.filter(a => !a.device || a.status !== 'logged_in');
      const availableDevices = this.devices.filter(d => !this.accounts.some(a => a.device === d.real_serial && a.status === 'logged_in'));

      const toAssign = Math.min(unassigned.length, availableDevices.length);

      // Assign accounts to devices and open the correct app
      const serialsByPlatform = {};
      for (let i = 0; i < toAssign; i++) {
        const acc = unassigned[i];
        const dev = availableDevices[i];
        acc.device = dev.real_serial;
        acc.deviceIndex = `#${this.devices.indexOf(dev) + 1}`;
        acc.status = 'assigned';

        // Group by platform for batch ADB commands
        if (!serialsByPlatform[acc.platform]) {
          serialsByPlatform[acc.platform] = [];
        }
        serialsByPlatform[acc.platform].push(dev.real_serial);
      }

      // Open the correct app on each device
      for (const [platform, serials] of Object.entries(serialsByPlatform)) {
        const p = this.platforms.find(x => x.id === platform);
        if (p) {
          this.$service.adb_command({
            serials: serials,
            args: ['shell', 'am', 'start', '-n', p.activity]
          });
        }
      }

      this.saveAccounts();
      await this.$emiter('NOTIFY', {
        type: 'success',
        message: `Assigned ${toAssign} accounts to devices and opened login screens. ${Math.max(0, unassigned.length - toAssign)} unassigned.`,
        timeout: 3000
      });
    },
    statusClass(status) {
      switch (status) {
        case 'logged_in': return 'badge-success';
        case 'assigned': return 'badge-info';
        case 'error': return 'badge-error';
        default: return 'badge-ghost';
      }
    },
    async saveAccounts() {
      await setItem('tikfarm_accounts', JSON.stringify(this.accounts));
    },
  },
  async mounted() {
    try {
      const stored = await getItem('tikfarm_accounts');
      if (stored) {
        this.accounts = JSON.parse(stored);
      }
    } catch (e) {
      console.warn('Failed to load accounts', e);
    }
  },
}
</script>
