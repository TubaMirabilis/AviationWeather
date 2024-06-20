if (args.Length == 0)
{
    Console.WriteLine("Please provide an ICAO code.");
    return;
}
var icao = args[0].ToUpper();
if (string.IsNullOrWhiteSpace(icao))
{
    Console.WriteLine("Invalid ICAO code.");
    return;
}
var url = $"https://aviationweather.gov/api/data/metar?ids={icao}&format=raw&taf=false";
var client = new HttpClient();
var response = await client.GetAsync(url);
var content = await response.Content.ReadAsStringAsync();
Console.WriteLine(content);
