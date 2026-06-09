# Tổng quan dự án

Bạn muốn hiểu dự án hãy học cách đọc code. Nhưng muốn hiểu luồng logic hãy đọc tài liệu

**Lưu ý:** Tài liệu được viết trên nguồn "tin tôi đi bạn". Mình không chịu trách nhiệm cho bất kỳ thiếu xót nào
ảnh hưởng đến tư duy, cảm xúc cá nhân hay code của bạn. Cân nhắc trước khi đọc. Nếu cảm thấy tài liệu chưa
đúng đừng ngần ngại viết code, tạo Pull Request và đóng góp lại cho dự án. Cảm ơn bạn đã đọc.

## Tổng quan kỹ thuật

### Cấu hình dự án

Xem code ở thư mục [settings](../src/config/settings.rs)

### Xử lý chuỗi

Sử dụng thư viện vi chỉ hỗ trợ Telex và VNI.

Sao phải hỗ trợ bộ gõ khác trong khi bạn có thể học kiểu gõ mới thay vì
dành nhiều giờ sửa lỗi phần mềm?

Xem code ở file [input_mode](../src/config/input_mode.rs)

Tham khảo ở [đây](https://github.com/ZeroX-DG/vi-rs)

### Systray

Sử dụng thư viện ksni theo chuẩn của KDE/freedesktop. Dùng để hiển thị icon.

Xem code ở file [tray](../src/systray/tray.rs)

Tham khảo ở [đây](https://docs.rs/ksni/latest/ksni/index.html)

### Xử lý key

Sử dụng thư viện xkbcommon.

Xem code ở file [keyboard](../src/systray/keyboard.rs)

Tham khảo ở [đây](https://docs.rs/xkbcommon/latest/xkbcommon/index.html)

## Cách tương tác Wayland compositor và ra input

Trước hết hãy đọc hiểu các chuẩn của Wayland protocols

[input-method-unstable-v2](https://wayland.app/protocols/input-method-unstable-vs2)

Chú thích có ghi
```
This protocol allows applications to act as input methods for compositors.
```
Dùng để biến app thành input method tương tác với compositors.

Ví dụ cho kẻ ngốc nghếch: Compositors là WM của bạn nhận text từ app. Cả compositors và app cần phải tự
viết code.

Cho dễ hiểu 2 người cần phải học tiếng Việt mới nói chuyện với nhau. 1 người học nói 1 người học nghe. 👨🔊🇻🇳👂👨
```
mimi-ime --text--> WM --text--> App
```

[text-input-unstable-v3](https://wayland.app/protocols/text-input-unstable-v3)

Chú thích có ghi
```
This protocol allows compositors to act as input methods and to send text to applications.
A text input object is used to manage state of what are typically text entry fields in the application.
```
Dùng để biến compositors thành input method tương tác với app.

Ví dụ cho kẻ ngốc nghếch: Compositors là WM của bạn gửi text tới app. Cả compositors và app cần phải tự
viết code.

Cho dễ hiểu 2 người cần phải học tiếng Việt mới nói chuyện với nhau. 1 người học nói 1 người học nghe. 👨🔊🇻🇳👂👨
```
WM --text--> App (firefox, alacritty...)
```


[virtual-keyboard-unstable-v1](https://wayland.app/protocols/virtual-keyboard-unstable-v1)

Chú thích có ghi
```
The virtual keyboard provides an application with requests which emulate the behaviour of a physical keyboard.
```
Giả lập bàn phím vật lý bằng software

Ví dụ cho kẻ ngốc nghếch: Khi user gõ một phím mà IME không cần xử lý (ví dụ Ctrl+C, F5, arrow keys...),
bạn không thể chỉ ignore nó — phải forward xuống app. Nhưng bạn đã grab keyboard rồi, phím đó không tự đến app
được nữa. Bạn cần virtual keyboard để giả phím đó để gõ xuống truyền tiếp qua app.

Giống như bảo vệ toà nhà ai có việc thì xử lý rồi mới cho vào, ai không có việc thì cho vào thẳng. 👮‍♂️
```
1. Keyboard → IME (grab)
2a. IME xử lý → commit_string → WM → App
2b. IME không xử lý → virtual_keyboard → WM → App
```

Xem code ở thư mục [systray](../src/input_method/wayland.rs)

## Tổng kết

Hãy hình dung thế này.

1. Người dùng mở IME có tray để cấu hình hoặc thoát app.
2. Người dùng gõ phím và IME nhận ký tự chuyển text sang tiếng việt.
3. Engine dùng các thư viện vi xử lý text sang tiếng Việt.
4. Engine dùng wayland protocol gửi text cho compositor
5. Compositor relay xuống App.
6. App hiện text tiếng Việt cho người dùng.

Bạn đã hiểu chưa? Nếu bạn chưa hiểu cũng không sao cả, vì mình cũng không hiểu gì cả. Tất cả chúng ta cũng
không cần phải hiểu máy tính. Hãy cùng nhau đập phá máy tính, học cách viết chữ tay và nói chuyện với nhau. Hurrayy!!!

Nhưng nếu bạn vẫn muốn hiểu hãy tiếp tục vọc mã nguồn và bạn có thể sẽ hiểu? Đừng ngần ngại gia nhập room chat để hiểu thêm nhé hay chỉ đơn giản tán dóc.

[#mimi-ime:matrix.org](https://matrix.to/#/#mimi-ime:matrix.org)
