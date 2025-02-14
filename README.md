# İTÜ Keplerbot

## Nedir?

İTÜ Keplerbot önceden zamanlanmış bir şekilde, herhangi bir tarayıcı kullanmadan tamamen HTTP üzerinden ders seçmenize yarar.

## Nasıl Kullanılır

1. Öncelikle sayfanın sağındaki 'Releases' kısmından uygulamanın son versiyonunu indirin. Windows kullanıyorsanız `itu-keplerbot.exe` adlı dosyayı, Linux kullanıyorsanız ise `itu-keplerbot` adlı dosyayı indirmelisiniz.

> [!NOTE]  
> MacOS kullanan arkadaşlar uygulamayı [kendileri derlemelidirler](#derlemek-için). Maalesef GitHub ücretsiz bir şekilde MacOS runner sağlamıyor :(

2. Şimdi gerekli bilgileri programa girmelisiniz. Bunu komut satırını kullanarak veya elle girerek yapabilirsiniz.

### Komut Satırı İle Bilgi Girme ***ÖNERİLİR***

Aşağıdaki komut ile bilgileriniz girebilirsiniz

```bash
itu-keplerbot make-config -u <KULLANICI_ADI> -p -<ŞİFRE> -t <SEÇİM_SAATİ> --crn <ALINACAK_CRNLER> --scrn <BIRAKILACAK_CRNLER>
```

CRN almak veya bırakmak istemiyorsanız `--crn` / `--scrn` alanlarını yazmanıza gerek yoktur.

> [!WARNING]  
> Saat kısmını belirtilen formatta girmelisiniz -> "YIL-AY-GÜN SAAT:DAKİKA:SANİYE"


#### Örnek Komut

```bash
itu-keplerbot make-config -u bicer22 -p 123şifre -t "2025-02-10 14:00:00" --crn 22612,22614,22609 --scrn 20399
```

Örneğin, eğer ders bırakmak istemiyorsanız:

```bash
itu-keplerbot make-config -u bicer22 -p 123şifre -t "2025-02-10 14:00:00" --crn 22612,22614,22609
```

Bu komut programı çalıştırdığınız yerde `config.json` adında bir dosya oluşturacaktır. Artık uygulama çalıştırılmaya hazır.

### Manuel Bilgi Girme

İndirdiğiniz program ile aynı klasörde `config.json` adında bir dosya oluşturun.

Oluşturulan `config.json` aşağıdaki şablona uyacak şekilde doldurulmalıdır.

```json
{
  "username": "<KULLANICI_ADI>",
  "password": "<ŞİFRE>",
  "time": "<YIL-AY-GÜN>T<SAAT:DAKİKA:SANİYE>+03:00", // Tarih ile saat arasındaki 'T', ve +03:00 öğelerine dikkat edin!
  "crn_list": ["<CRN>", "<CRN>", "<CRN>" ...], // Tırnak içerisinde, virgüllerle ayrılmış
  "scrn_list": ["<CRN>" ...]
}
```

#### Örnek `config.json`

```json
{
  "username": "bicer22",
  "password": "123şifre",
  "time": "2025-02-10T14:00:00+03:00", 
  "crn_list": ["22612", "22614", "22609"],
  "scrn_list": ["20399"]
}
```

Örneğin, eğer ders bırakmak istemiyorsanız:

```json
{
  "username": "bicer22",
  "password": "123şifre",
  "time": "2025-02-10T14:00:00+03:00", 
  "crn_list": ["22612", "22614", "22609"],
  "scrn_list": []
}
```

3. Bilgilerinizi girdikten sonra artık programı çalıştırmaya hazırsınız. 

`config.json` dosyanızın uygulama ile aynı klasörde olduğundan emin olun. Komut satırından aşağıdaki komut ile uygulamayı çalıştırın.

```bash
itu-keplerbot run
```

Eğer JSON bilgi dosyanız farklı bir konumdaysa aşağıdaki komutu kullanabilirsiniz:

```bash
itu-keplerbot run --config <JSON_DOSYASI_KONUMU>

```

> [!CAUTION]
> Eğer bilgilere girdiğiniz saat geçmişteyse, uygulama hemen HTTP request atmaya başlayacaktır.

4. Uygulama artık çalışıyor, bol şans :pray: :rocket: !

## Teşekkür

Ata'nın yaptığı uygulama benim için önemli bir kaynaktı, teşekkürler :) [Yıldızlamayı unutmayın!](https://github.com/AtaTrkgl/itu-ders-secici)


## Derlemek İçin

1. Eğer yüklü değilse, Rust ve gerekli aletleri indrin -> [Buradan indirebilirsiniz](https://www.rust-lang.org/tools/install)

2. Repo'yu klonlayın

```bash
git clone https://github.com/Utkub24/itu-keplerbot.git
```

3. Komut satırından klonladığınız klasöre gidip `cargo` kullanarak derleyebilirsiniz.

Sadece derlemek için:

```bash
cargo build
```

Derleyip çalıştırmak için:

```bash
cargo run
```
