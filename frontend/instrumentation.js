import { generateXML } from '@/utils/generate';

export async function register() {
  // Generate static site map and RSS feed
  console.log("Running instrumentation script...")
  await generateXML();
}