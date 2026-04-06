<template>
  <div class="fixed inset-0 z-[9999] flex items-center justify-center bg-base-300">
    <div class="card w-96 bg-base-100 shadow-2xl border border-base-content/10">
      <div class="card-body items-center text-center">
        <!-- Logo -->
        <div class="mb-4">
          <img :src="logoSrc" class="h-16 w-auto mx-auto" alt="TrustFarm" />
          <h1 class="text-2xl font-bold mt-3">
            <span class="text-primary">Trust</span><span class="text-base-content">Farm</span>
          </h1>
          <p class="text-sm text-base-content/50 mt-1">Phone Farm Manager</p>
        </div>

        <!-- Error -->
        <div v-if="errorMsg" class="alert alert-error text-sm w-full">
          <span>{{ errorMsg }}</span>
        </div>

        <!-- Login Form -->
        <div v-if="mode === 'login'" class="w-full space-y-3">
          <input v-model="email" type="email" placeholder="Email" class="input input-bordered w-full" @keyup.enter="login" />
          <input v-model="password" type="password" placeholder="Contrasena" class="input input-bordered w-full" @keyup.enter="login" />
          <button @click="login" class="btn btn-primary w-full" :disabled="loading">
            <span v-if="loading" class="loading loading-spinner loading-sm"></span>
            {{ loading ? 'Ingresando...' : 'Iniciar sesion' }}
          </button>

          <div class="divider text-xs text-base-content/30">o</div>

          <button @click="loginWithGoogle" class="btn btn-outline w-full gap-2" :disabled="loading">
            <svg class="w-5 h-5" viewBox="0 0 24 24"><path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92a5.06 5.06 0 01-2.2 3.32v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.1z"/><path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/><path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/><path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/></svg>
            Continuar con Google
          </button>

          <p class="text-sm text-base-content/50">
            No tienes cuenta? <a @click="mode = 'register'" class="text-primary cursor-pointer hover:underline">Registrate</a>
          </p>
        </div>

        <!-- Register Form -->
        <div v-if="mode === 'register'" class="w-full space-y-3">
          <input v-model="email" type="email" placeholder="Email" class="input input-bordered w-full" @keyup.enter="register" />
          <input v-model="password" type="password" placeholder="Contrasena (min 6 caracteres)" class="input input-bordered w-full" @keyup.enter="register" />
          <button @click="register" class="btn btn-primary w-full" :disabled="loading">
            <span v-if="loading" class="loading loading-spinner loading-sm"></span>
            {{ loading ? 'Creando cuenta...' : 'Crear cuenta' }}
          </button>
          <p class="text-sm text-base-content/50">
            Ya tienes cuenta? <a @click="mode = 'login'" class="text-primary cursor-pointer hover:underline">Inicia sesion</a>
          </p>
        </div>

        <div class="divider text-xs text-base-content/30">TrustMind Ecosystem</div>
        <p class="text-xs text-base-content/40">Usa la misma cuenta de TrustInsta o TrustFace</p>
      </div>
    </div>
  </div>
</template>

<script>
import { getSupabase, getUserTier } from '../lib/supabase.js'
import { WebviewWindow } from '@tauri-apps/api/window'
import logoUrl from '../assets/app-icon.png'

export default {
  name: 'TrustLogin',
  emits: ['authenticated'],
  data() {
    return {
      mode: 'login',
      email: '',
      password: '',
      loading: false,
      errorMsg: '',
      logoSrc: logoUrl,
    }
  },
  async mounted() {
    // Try to restore session
    const supabase = getSupabase()
    const { data: { session } } = await supabase.auth.getSession()
    if (session?.user) {
      const tier = await getUserTier(session.user.id)
      this.$emit('authenticated', { user: session.user, tier })
    }
  },
  methods: {
    async login() {
      if (!this.email || !this.password) return
      this.loading = true
      this.errorMsg = ''
      try {
        const supabase = getSupabase()
        const { data, error } = await supabase.auth.signInWithPassword({
          email: this.email,
          password: this.password,
        })
        if (error) {
          this.errorMsg = error.message === 'Invalid login credentials'
            ? 'Email o contrasena incorrectos'
            : error.message
          return
        }
        const tier = await getUserTier(data.user.id)
        this.$emit('authenticated', { user: data.user, tier })
      } catch (err) {
        this.errorMsg = 'Error de conexion'
      } finally {
        this.loading = false
      }
    },
    async loginWithGoogle() {
      this.loading = true
      this.errorMsg = ''
      try {
        const supabase = getSupabase()
        const { data, error } = await supabase.auth.signInWithOAuth({
          provider: 'google',
          options: {
            skipBrowserRedirect: true,
            redirectTo: 'https://www.trustmind.online/auth/callback',
          },
        })
        if (error) {
          this.errorMsg = error.message
          this.loading = false
          return
        }

        // Open OAuth URL in a Tauri webview window
        const oauthWindow = new WebviewWindow('google-oauth', {
          url: data.url,
          title: 'Iniciar sesion con Google',
          width: 500,
          height: 700,
          center: true,
          resizable: false,
        })

        // Poll for session (Supabase will set it when OAuth completes)
        const pollInterval = setInterval(async () => {
          const { data: sessionData } = await supabase.auth.getSession()
          if (sessionData?.session?.user) {
            clearInterval(pollInterval)
            try { oauthWindow.close() } catch {}
            const tier = await getUserTier(sessionData.session.user.id)
            this.$emit('authenticated', { user: sessionData.session.user, tier })
          }
        }, 1500)

        // Stop polling after 2 minutes
        setTimeout(() => {
          clearInterval(pollInterval)
          this.loading = false
        }, 120000)

        // If window closes without auth
        oauthWindow.onCloseRequested?.(() => {
          clearInterval(pollInterval)
          this.loading = false
        })
      } catch (err) {
        this.errorMsg = 'Error al conectar con Google'
        this.loading = false
      }
    },
    async register() {
      if (!this.email || !this.password) return
      if (this.password.length < 6) {
        this.errorMsg = 'La contrasena debe tener al menos 6 caracteres'
        return
      }
      this.loading = true
      this.errorMsg = ''
      try {
        const supabase = getSupabase()
        const { error } = await supabase.auth.signUp({
          email: this.email,
          password: this.password,
        })
        if (error) {
          this.errorMsg = error.message
          return
        }
        this.errorMsg = ''
        this.mode = 'login'
        alert('Cuenta creada. Revisa tu email para verificar tu cuenta.')
      } catch {
        this.errorMsg = 'Error de conexion'
      } finally {
        this.loading = false
      }
    },
  },
}
</script>
