export type Lab = { id:string; slug:string; title:string; provider:string; difficulty:string; duration_minutes:number; description:string; price_minor:number; currency:string; objectives:string[]; tasks:{id:string;title:string;description:string;points:number}[] };
const base = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:8080";
export async function getLabs(): Promise<Lab[]> { const r = await fetch(`${base}/api/v1/labs`, { cache:"no-store" }); if(!r.ok) throw new Error("Could not load labs"); return r.json(); }
export async function getLab(slug:string): Promise<Lab> { const r = await fetch(`${base}/api/v1/labs/${slug}`, { cache:"no-store" }); if(!r.ok) throw new Error("Could not load lab"); return r.json(); }
