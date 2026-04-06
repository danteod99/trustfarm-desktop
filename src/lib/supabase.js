import { createClient } from '@supabase/supabase-js'

const SUPABASE_URL = 'https://jlxaubqvgjahcsnotvih.supabase.co'
const SUPABASE_ANON_KEY = 'sb_publishable_ooYxQ-GHz2mayTFJPIswyA_EAyP_gRi'

let instance = null

export function getSupabase() {
  if (!instance) {
    instance = createClient(SUPABASE_URL, SUPABASE_ANON_KEY, {
      auth: {
        persistSession: true,
        autoRefreshToken: true,
        detectSessionInUrl: false,
        storageKey: 'trustfarm-auth',
      },
    })
  }
  return instance
}

/**
 * Check user tier from tm_subscriptions table
 * Returns 'pro' or 'free'
 */
export async function getUserTier(userId) {
  const supabase = getSupabase()
  try {
    const { data, error } = await supabase
      .from('tm_subscriptions')
      .select('tier, product, expires_at')
      .eq('user_id', userId)
      .eq('tier', 'pro')
      .order('expires_at', { ascending: false })
      .limit(5)

    if (error || !data || data.length === 0) return 'free'

    const now = new Date().toISOString()
    const valid = data.find(sub => {
      const notExpired = !sub.expires_at || sub.expires_at > now
      const validProduct = !sub.product || sub.product === 'trustfarm' || sub.product === 'bundle' || sub.product === 'all'
      return notExpired && validProduct
    })

    return valid ? 'pro' : 'free'
  } catch {
    return 'free'
  }
}
