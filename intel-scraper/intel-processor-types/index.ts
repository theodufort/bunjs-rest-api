import puppeteer, { executablePath } from "puppeteer";
import cheerio from "cheerio";
import cheerioTableparser from "cheerio-tableparser";

//https://en.wikipedia.org/wiki/Comparison_of_Intel_processors
async function uploadToDB() {}

async function main() {
  // Launch the browser and open a new blank page
  const browser = await puppeteer.launch({
    executablePath: "/usr/bin/chromium",
    headless: "new",
  });
  const page = await browser.newPage();

  // Navigate the page to a URL
  await page.goto(
    "https://en.wikipedia.org/wiki/Comparison_of_Intel_processors",
    {
      waitUntil: "networkidle0",
    }
  );
  const html = await page.evaluate(() => document.querySelector("*").outerHTML);

  const $ = cheerio.load(html);
  cheerioTableparser($);
  var data = $(".wikitable").parsetable(true, true, true);
  const rows = data[0].map((_, columnIndex) =>
    data.map((row) => row[columnIndex])
  );
  //Remove last row (headers)
  rows.pop();
  const headers = rows[0];
  await browser.close();
  console.log(headers);
  console.log(rows[0]);
  console.log(rows[1]);
  console.log(rows[2]);
}
await main();
