import puppeteer, { executablePath } from "puppeteer";
import cheerio from "cheerio";
import cheerioTableparser from "cheerio-tableparser";

async function uploadToDB() {}

async function main() {
  // Launch the browser and open a new blank page
  const browser = await puppeteer.launch({
    executablePath: "/usr/bin/chromium",
    headless: "new",
  });
  const page = await browser.newPage();

  // Navigate the page to a URL
  await page.goto("https://en.wikipedia.org/wiki/Table_of_AMD_processors", {
    waitUntil: "networkidle0",
  });
  const html = await page.evaluate(() => document.querySelector("*").outerHTML);

  const $ = cheerio.load(html);
  cheerioTableparser($);
  var data = $(".wikitable").parsetable(true, true, true);
  const rows = data[0].map((_, columnIndex) =>
    data.map((row) => row[columnIndex])
  );
  //Remove last row (headers)
  rows.pop();
  const headers = rows[0].map((x, index) => {
    if (x == rows[1][index]) {
      return x;
    } else {
      return rows[1][index];
    }
  });
  await browser.close();
  console.log(headers);
}
await main();
