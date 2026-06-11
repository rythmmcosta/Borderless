const OWNER = process.env.GITHUB_OWNER ?? 'rythmmcosta'
const REPO  = process.env.GITHUB_REPO  ?? 'Borderless'

export type Platform = 'windows' | 'macos' | 'linux' | 'android'

export interface PlatformAsset {
  platform:  Platform
  label:     string
  ext:       string
  url:       string
  size:      number
}

export interface Release {
  version:     string
  name:        string
  publishedAt: string
  notes:       string
  githubUrl:   string
  assets:      PlatformAsset[]
  prerelease:  boolean
}

const ASSET_PATTERNS: { re: RegExp; platform: Platform; label: string; ext: string }[] = [
  { re: /_x64-setup\.exe$/i,    platform: 'windows', label: 'Windows 64-bit Installer', ext: '.exe'      },
  { re: /\.msi$/i,              platform: 'windows', label: 'Windows MSI Package',       ext: '.msi'      },
  { re: /\.exe$/i,              platform: 'windows', label: 'Windows Portable',           ext: '.exe'      },
  { re: /\.dmg$/i,              platform: 'macos',   label: 'macOS (Apple Silicon)',      ext: '.dmg'      },
  { re: /\.app\.tar\.gz$/i,     platform: 'macos',   label: 'macOS (tar.gz)',             ext: '.tar.gz'   },
  { re: /\.AppImage$/i,         platform: 'linux',   label: 'Linux AppImage',             ext: '.AppImage' },
  { re: /amd64\.deb$/i,         platform: 'linux',   label: 'Linux Debian/Ubuntu (.deb)', ext: '.deb'      },
  { re: /\.rpm$/i,              platform: 'linux',   label: 'Linux RPM',                  ext: '.rpm'      },
  { re: /app-release\.apk$/i,   platform: 'android', label: 'Android APK',                ext: '.apk'      },
  { re: /app-release\.aab$/i,   platform: 'android', label: 'Android App Bundle',         ext: '.aab'      },
]

interface GhAsset { name: string; browser_download_url: string; size: number }
interface GhRelease { tag_name: string; name: string; published_at: string; body: string | null; html_url: string; assets: GhAsset[]; prerelease: boolean; draft: boolean }

function mapAssets(raw: GhAsset[]): PlatformAsset[] {
  const out: PlatformAsset[] = []
  const seen = new Set<string>()

  for (const a of raw) {
    for (const p of ASSET_PATTERNS) {
      if (!p.re.test(a.name)) continue
      const key = `${p.platform}:${p.ext}`
      if (seen.has(key)) continue
      seen.add(key)
      const url: string = a.browser_download_url ?? ''
      try {
        const parsed = new URL(url)
        if (!['github.com', 'objects.githubusercontent.com'].includes(parsed.hostname)) continue
      } catch { continue }
      out.push({ platform: p.platform, label: p.label, ext: p.ext, url, size: a.size })
      break
    }
  }
  return out
}

async function ghFetch(path: string) {
  const headers: Record<string, string> = { Accept: 'application/vnd.github+json' }
  if (process.env.GITHUB_TOKEN) headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`

  const res = await fetch(`https://api.github.com/repos/${OWNER}/${REPO}${path}`, {
    headers,
    next: { revalidate: 300 }, // 5-minute ISR cache
  })
  if (!res.ok) throw new Error(`GitHub API ${res.status}: ${path}`)
  return res.json()
}

export async function getLatestRelease(): Promise<Release | null> {
  try {
    const d = await ghFetch('/releases/latest')
    return { version: d.tag_name, name: d.name, publishedAt: d.published_at, notes: d.body ?? '', githubUrl: d.html_url, assets: mapAssets(d.assets), prerelease: d.prerelease }
  } catch {
    return null
  }
}

export async function getAllReleases(): Promise<Release[]> {
  try {
    const data: GhRelease[] = await ghFetch('/releases?per_page=20')
    return data
      .filter((r) => !r.draft)
      .map((d) => ({ version: d.tag_name, name: d.name, publishedAt: d.published_at, notes: d.body ?? '', githubUrl: d.html_url, assets: mapAssets(d.assets), prerelease: d.prerelease }))
  } catch {
    return []
  }
}

const VERSION_RE = /^v?\d{1,4}\.\d{1,4}\.\d{1,4}(-[a-zA-Z0-9._-]{1,40})?$/

export async function getReleaseByVersion(version: string): Promise<Release | null> {
  if (!VERSION_RE.test(version)) return null
  try {
    const d = await ghFetch(`/releases/tags/${encodeURIComponent(version)}`)
    return { version: d.tag_name, name: d.name, publishedAt: d.published_at, notes: d.body ?? '', githubUrl: d.html_url, assets: mapAssets(d.assets), prerelease: d.prerelease }
  } catch {
    return null
  }
}

export function fmtBytes(n: number): string {
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(0)} KB`
  return `${(n / 1024 / 1024).toFixed(1)} MB`
}

export function fmtDate(iso: string): string {
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })
}
