# Tại sao có Mimi IME

Bài viết này không nhằm mục đích chỉ trích bất kỳ dự án nào. Đây là những đúc kết kỹ thuật sau quá trình vận hành,
debug các vấn đề xung đột IME trên Linux. Mục đích của Mimi IME là một giải pháp thuần Wayland, ưu tiên sự ổn định
thay vì các hacky workaround vốn gây xung đột với các ứng dụng hiện đại.

Rồi mình cũng nói là mình có thể sai trong 1 vài quan điểm nên hy vọng bạn thông cảm và mình sẽ cố gắng tập trung
vào kỹ thuật hy vọng bạn sẽ hứng thú đọc tiếp ❤️.

## Tóm tắt lịch sử Input Method

Về lịch sử IME thì người dùng Việt nam tiếp cận với Unikey nên khi qua Linux họ vẫn mang theo tư tưởng Backspace là tốt
gõ rất mượt commit liên tục nhưng họ quên mất rằng đây là Linux nơi sẽ có tiêu chuẩn khắt khe hơn. Và họ nghĩ ra rất
nhiều ý tưởng để nhái lại 1 phần của Unikey. Mình sẽ không bàn tới các dự án đã chết trước đó vì mình chưa dùng Linux
trước năm 2018 nhiều lắm nên mong bạn thông cảm. Mình biết thì ibus-bamboo có nhiều chế độ gõ bằng cách tạo lại
fake-backspace gì đó trên Xorg, Fcitx5-lotus hiện tại đang xài Uinput. Và kết quả rất nhiều bài toán race condition
xảy ra xung đột giữa phần mềm và bàn phím chúng ta gõ xảy ra.

Nghe không đáng tin nhỉ? Mình cũng thấy vậy nên mình sẽ đưa ra vài bằng chứng thuyết phục hơn. Chính fcitx5 developer
cũng có nói về vấn đề này. Và maintainer cũ của ibus-bamboo cũng có nói về vấn đề này. Cũng có khá nhiều bug xoay quanh
app hoạt động không tốt khi xài chúng.

https://github.com/BambooEngine/bamboo-core/issues/2#issuecomment-1146852396

```
wengxt

@luongthanhlam @vuongtuha I'm not a big fan of adding any hacky typing mode in fcitx5-bamboo. I know ibus-bamboo has lots of such code, but please don't the same for fcitx5-bamboo.
My opinion on the adding existing mode in ibus-bamboo:

using surrounding text is ok with me, but usually doesn't worth the effort due to the poor implementation on linux. I would only use it as some auxiliary data.
typing mode sending fake X key would never be acceptable - it doesn't work on wayland anyway.
non-preedit popup window can be easily toggled in fcitx (Ctrl+Alt+P), so no need to add that. We know preedit can be problematic in certain cases or a workaround for certain problem, so we have this support in framework instead of in engine.
The preferable route should always be fixing the problematic program themselves. Adding workaround like that only pushes the burden to user. I always try to keep the amount of such hack as low as possible. We maintain like almost 20~ different fcitx engines, adding hack to certain engine to me is not a right way to go.

But Feel free to report any issue you have to fcitx. Even if eventually we found it's an application problem, fcitx developers will be willing to look into the application code find a fix, or report with more details to help application owner to fix.
```

https://github.com/BambooEngine/ibus-bamboo/issues/590#issuecomment-3762683651

```
goatastronaut0212

Mình sẽ rất vui nếu ai đó truyền thông điệp của mình cho những người khác trong cộng đồng nguồn mở hay gì đó vì mình cũng không xài mạng xã hội để họ có thể hiểu thêm về tình trạng input method ở Linux.

Cho những người lười đọc thì đây vắn tắt của mình.

ibus-bamboo có ngừng phát triển là vì xây dựng trên nền móng không vững như thư viện godbus, wl là nguyên nhân chính khiến nó khó duy trì chứ không phải Ibus là vấn đề.
fcitx5-unikey đã hoàn thiện rồi ai muốn phát minh cái mới lại thì việc của họ. Nhưng nếu cần 1 thứ làm tốt việc thì là fcitx5-unikey.
Nếu bạn muốn dự án mới nào sống tiếp vui lòng hãy tuân thủ tiêu chuẩn từ upstream. Đừng cố biến dự án trở thành 1 khu rừng để ôm đồm mọi chuyện và vắt kiệt sức bản thân.
Đây là lời văn dài hơn nếu bạn muốn nghe mình giải thích.

@hien-ngo29 Mình đã có theo dõi bạn 1 thời gian và dự án mới của bạn thì khi quay lại mình có thể khẳng định những gì mình nói trước đó là không hề nhầm lẫn. Mình đã quản lý và xem vấn đề kỹ thuật của ibus-bamboo trong 1 thời gian dài. Bằng chứng cho thấy rõ ràng nhất là báo cáo lỗi bạn gửi ibus/ibus#2842. Vấn đề mình đã nói là godbus có vấn đề và nó sử dụng dbus để tương tác với compositor, mấu chốt ở đây không có ai rảnh để đi cài 1 extension ngoài luồng chỉ để hiện những thứ đó. #374 (comment) có đề cập đến comment GNOME. Giải pháp rất hacky và tạm thời, đúng vậy nó vẫn hoạt động nhưng trong bao lâu nữa? Và chúng ta đang đi ngược với những gì tiêu chuẩn từ upstream (các phần phần mềm và tiêu chuẩn phía trên) bạn không thể chèo thuyền ngược mãi được. Fcitx5-unikey? Họ không có thứ đó, mọi thứ vẫn hoạt động như bình thường. Không có dbus, không còn cài extension và mọi thứ chỉ hoạt động.

Ngoài lề 1 chút thì vấn đề nghiêm trọng khác mình rất sợ các dự án mới sẽ lại bắt đầu lại trở thành 1 khu rừng từ chối tiêu chuẩn của upstream không chịu nghe họ hay đóng góp ngược lại và tiếp tục chạy ngược dòng. Tệ hơn họ có thể có nguy cơ vào lại vết xe đổ của dự án này ôm đồm quá nhiều nhưng không thể hoàn thành. Mình hy vọng nếu họ có làm họ sẽ biết phải làm gì để tuân thủ các tiêu chuẩn upstream. Đầu tiên như mình đã nói Chế độ gõ trong Ibus pre-edit là chế độ 1, vấn đề có thể chúng ta cần để tâm đến mọi người dùng cứ luôn phàn nàn cứ phải gõ space cách ra để làm chi vậy sao không dùng chế độ khác commit luôn đi rồi backspace nhưng nếu như nó làm thế nhỡ gây crash app thì sao? #541. Đúng pre-edit không hoàn hảo nhưng nó thực sự làm được việc đơn giản đó, nếu bạn đọc tiêu chuẩn wayland bạn sẽ hiểu tại sao họ chỉ có pre-edit mà không có những thứ khác https://wayland.app/protocols/text-input-unstable-v3#zwp_text_input_v3. Bạn có thể yêu thích backspace xoá chữ đi và giải pháp của Windows nhưng đây là Wayland, bạn là dự án phía dưới phụ thuộc vào họ và tiêu chuẩn của họ phải được đặt lên hàng đầu. Mình nghĩ nếu chọn ra bộ gõ tiếng Việt hoàn hảo nhất ở thời điểm hiện tại tuân thủ tiêu chuẩn đó thì ngoài fcitx5-unikey ra mình không thể nghĩ ra ai khác dù trước đó mình có không thích nó nhưng giờ thì mình hiểu vì sao nó là 1 sản phẩm đã hoàn thiện.

Mình khá là burnout khi phải tiếp tục phát minh bánh xe bộ gõ tiếng Việt chỉ để "nhái lại 1 phần sức mạnh của Unikey Windows" và tiếp tục ngược dòng tiêu chuẩn chính. Hãy để fcitx5-unikey là tiêu chuẩn mới của Unikey trên Linux hoặc bất kỳ bộ gõ nào khác ngoài Ibus, Fcitx5 tiếng Việt nào đó tuân thủ pre-edit hay tiêu chuẩn input method của Wayland. Vấn đề đã được giải quyết, thực tế mọi người đã có giải pháp xong xuôi rồi đừng quá tốn thời gian nữa hãy đi phát triển nhưng thứ mới mà bạn hứng thú và vui vẻ hơn.
```

Các bugs bên Ibus-bamboo và fcitx5-lotus
https://github.com/BambooEngine/ibus-bamboo/issues/541
https://github.com/LotusInputMethod/fcitx5-lotus/issues/267
https://github.com/LotusInputMethod/fcitx5-lotus/issues/162
https://github.com/LotusInputMethod/fcitx5-lotus/issues/124

Khoan đã vậy nó có đúng là lỗi không? Mình không đoán được rõ nhưng mình có thể khẳng định những gì những người trước có
nói về vấn đề này là đúng. Nhưng bạn biết điều buồn cười nhất là gì không chúng ta than phiền pre-edit quá tệ nhưng không
giải thích được rõ ràng trong khi cơ chế đôi khi hành vi người dùng gõ `space`, `Enter`, `.` là đã commit text xong hoặc
có thể thêm text suggestion hoặc  cách nào đó kiểm tra lại từ của người dùng gõ đúng chưa. Điều buồn cười nhất Android có
pre-edit nhưng chả ai than phiền cả. Chúng ta đã hoàn thành xong input method nhưng thay vì không chịu thay đổi tư duy
chúng ta hack cho đến khi nào phần mềm chỉ có bug bug và bug. Burnout chán rồi nghỉ.

Nguồn Android

https://developer.android.com/reference/android/inputmethodservice/InputMethodService?hl=en

Xem kỹ method getCurrentInputConnection(), setComposingText() và commitText()

## Mimi IME

Thực ra Mimi IME ra đời là dự án cá nhân chết yểu trong 10 tháng. Được mình gom ý tưởng rồi làm nó hoạt động.
Ừm chỉ có nhiêu đó thôi chả có gì thêm cả. Nếu bạn cần thì đây.

+ Ít phụ thuộc vào các dependencies đỡ làm bạn năng máy.
+ Tuân thủ theo tiêu chuẩn của Wayland không bất kỳ giải pháp Backspace ngược dòng hay UInput nào.
+ Viết bằng Rust 🦀 (điểm mạnh mình đoán thế?)
+ Thay vì thành Ibus/Fcitx5 engine thì Mimi IME đã sánh vai với  Ibus/Fcitx5 (Không phải điểm cộng lắm)

Điểm trừ

- Vẫn đang nghiên cứu
- Không hỗ trợ GNOME/KDE (vì kiểu gì cũng phải bọc Ibus/Fcitx5 và thêm code làm dự án phình to)

## Tổng kết

Chung quy lại nếu bạn muốn xài Mimi IME bạn sẽ không muốn xài GNOME/KDE vì IME của mình tuân theo chuẩn chung và
GNOME/KDE hiện chưa muốn implement chuẩn mới input-method-v2 nên hãy sử fcitx5 hay Ibus vẫn tốt trên đó. Và nếu bạn cảm thấy
không có gì phải sử dụng Mimi IME thì bạn cũng không cần phải chuyển vì nó chỉ là 1 bộ gõ input method. Và Ibus cùng Fcitx5
đã làm rất tốt rồi. Nhưng hãy lưu ý về chuyện non pre-edit trên cái fcitx5/ibus mà mình nghĩ nó có thể làm crash app,
xung đột app hoạt động không đúng.

Đến đây là hết rồi. Cảm ơn bạn đã đọc. Chúc bạn có ngày tốt lành! ❤️ - theeasternfurry
