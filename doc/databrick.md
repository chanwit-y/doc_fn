ขอโทษครับ ลองใช้ path แบบนี้แทน:

**Method:** `PUT`

**URL:**
```
https://adb-7405616008575245.5.azuredatabricks.net/api/2.0/workspace-files/import-file/Users/chanwit_y%40banpu.co.th/drop-zone/my_file.pdf
```

> เปลี่ยน `@` เป็น `%40`

---

**ถ้ายังไม่ได้** ลองใช้ API นี้แทน:

**Method:** `POST`

**URL:**
```
https://adb-7405616008575245.5.azuredatabricks.net/api/2.0/workspace/import
```

**Headers:**
```
Authorization: Bearer <YOUR_DATABRICKS_TOKEN>
```

**Body:** เลือก **form-data** (ไม่ใช่ raw JSON)

| Key | Value |
|---|---|
| `path` | `/Users/chanwit_y@banpu.co.th/drop-zone/my_file.pdf` |
| `format` | `AUTO` |
| `content` | เลือก type เป็น **File** → เลือกไฟล์ PDF |
| `overwrite` | `true` |

> สำคัญ: ลบ header `Content-Type: application/json` ออก เพราะ Postman จะ set ให้เอง

---

**ถ้าทั้ง 2 วิธียังไม่ได้** แปลว่า Workspace ของคุณอาจไม่รองรับ file upload แบบนี้ ต้องใช้ **Unity Catalog Volumes** แทน

ลองวิธีแรก (encode `@` เป็น `%40`) ก่อนครับ แล้วบอกผล!