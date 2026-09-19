import Link from "next/link";

export function SiteHeader({ compact = false }: { compact?: boolean }) {
  return <header className="site-header">
    <Link className="brand" href="/">kloud<span>lab</span></Link>
    {!compact && <nav className="main-nav"><Link href="/labs">Labs</Link><Link href="/about">How it works</Link><Link href="/pricing">Pricing</Link></nav>}
    <div className="header-actions"><Link className="text-link hide-mobile" href="/login">Log in</Link><Link className="button small" href="/register">Start learning <span>↗</span></Link></div>
  </header>;
}

export function AppHeader() {
  return <header className="site-header app-header"><Link className="brand" href="/">kloud<span>lab</span></Link><nav className="main-nav"><Link href="/labs">Labs</Link><Link href="/dashboard">Dashboard</Link><Link href="/about">About</Link></nav><Link className="avatar" href="/dashboard">KL</Link></header>;
}
