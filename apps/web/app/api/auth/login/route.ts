import { NextRequest, NextResponse } from 'next/server'

const BACKEND = process.env.INTERNAL_API_URL ?? 'http://localhost:8080'

export async function POST(req: NextRequest) {
  let body: unknown
  try {
    body = await req.json()
  } catch {
    return NextResponse.json({ error: 'Invalid request body.' }, { status: 400 })
  }

  let res: Response
  try {
    res = await fetch(`${BACKEND}/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
      signal: AbortSignal.timeout(8000),
    })
  } catch {
    return NextResponse.json({ error: 'Backend unreachable.' }, { status: 502 })
  }

  if (!res.ok) {
    return NextResponse.json({ error: 'Invalid credentials.' }, { status: 401 })
  }

  let data: { token?: string }
  try {
    data = await res.json()
  } catch {
    return NextResponse.json({ error: 'Unexpected backend response.' }, { status: 502 })
  }

  if (!data.token) {
    return NextResponse.json({ error: 'No token in response.' }, { status: 502 })
  }

  const response = NextResponse.json({ ok: true })
  response.cookies.set('borderless_token', data.token, {
    httpOnly: true,
    secure: process.env.NODE_ENV === 'production',
    sameSite: 'lax',
    path: '/',
    maxAge: 60 * 60 * 24 * 7, // 7 days
  })
  return response
}
