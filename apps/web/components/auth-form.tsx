"use client";
import Link from "next/link";
import { FormEvent, useState } from "react";
const base = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:8080";
export function AuthForm({ mode }: { mode: "login" | "register" }) {
  const [email, setEmail] = useState(""); const [password, setPassword] = useState(""); const [error, setError] = useState(""); const [busy, setBusy] = useState(false);
  async function submit(event: FormEvent) { event.preventDefault(); setBusy(true); setError(""); try { const response = await fetch(`${base}/api/v1/auth/${mode}`, { method: "POST", credentials: "include", headers: { "content-type": "application/json" }, body: JSON.stringify({ email, password }) }); const body = await response.json(); if (!response.ok) throw new Error(body?.error?.message ?? "Something went wrong."); window.location.href = "/dashboard"; } catch (e) { setError(e instanceof Error ? e.message : "Unable to continue."); setBusy(false); } }
  return <form className="auth-form" onSubmit={submit}><label>Email address<input type="email" value={email} onChange={e => setEmail(e.target.value)} placeholder="you@company.com" required /></label><label>Password<input type="password" value={password} onChange={e => setPassword(e.target.value)} placeholder="At least 8 characters" minLength={8} required /></label>{error && <div className="form-error">{error}</div>}<button className="button full" disabled={busy}>{busy ? "Working…" : mode === "login" ? "Log in to KloudLab" : "Create your account"}</button><p className="form-foot">{mode === "login" ? <>New to KloudLab? <Link href="/register">Create an account</Link></> : <>Already have an account? <Link href="/login">Log in</Link></>}</p></form>;
}
