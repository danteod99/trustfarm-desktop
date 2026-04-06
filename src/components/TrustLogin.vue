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
