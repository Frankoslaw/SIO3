import * as fs from 'fs';

export async function GET() {
    var pdf = fs.readFileSync('./static/dom.pdf')

    return new Response(pdf, {
        status:200,
        headers: {
            "Content-type" : "application/pdf"
        }
    })
}
