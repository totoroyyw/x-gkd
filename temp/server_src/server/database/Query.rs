#[macro_use]
extern crate mysql;
use std::*;
include!("FileItem.rs");
include!("RequestItem.rs");
include!("DeviceItem.rs");
use mysql as my;

struct UserItem {
    id: i32,
    name: Option<String>,
    passwd: Option<String>,
}

struct FragmentItem {
    id: i32,
    path: Option<String>,
}

struct Query{
    pool:mysql::Pool
}
fn main() {
    println!("Hello, world!");
}

impl Query {
    pub fn queryFile_Bypathname(&self, path: Option<String>, name: Option<String>) -> FileItem{
        let selected_files: Vec<FileItem> =
        self.pool.prep_exec("SELECT * FROM file WHERE NAME = :name AND PATH = :path", 
                params!{"name" => name, "path" => path})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, path, attribute, time, noa, is_folder) = my::from_row(row);
                FileItem {
                    id: id,
                    name: name,
                    path: path,
                    attribute: attribute,
                    time: time,
                    noa: noa,
                    is_folder: is_folder,
                }
            }).collect()
        }).unwrap();
        return FileItem {
            id: selected_files[0].id,
            name: selected_files[0].name.clone(),
            path: selected_files[0].path.clone(),
            attribute: selected_files[0].attribute.clone(),
            time: selected_files[0].time.clone(),
            noa: selected_files[0].noa,
            is_folder: selected_files[0].is_folder,
        }
    }

    pub fn queryFile_Byid(&self, id: i32) -> FileItem{
        let selected_files: Vec<FileItem> =
        self.pool.prep_exec("SELECT * FROM file WHERE ID = :id", 
                params!{"id" => id})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, path, attribute, time, noa, is_folder) = my::from_row(row);
                FileItem {
                    id: id,
                    name: name,
                    path: path,
                    attribute: attribute,
                    time: time,
                    noa: noa,
                    is_folder: is_folder,
                }
            }).collect()
        }).unwrap();
        return FileItem {
            id: selected_files[0].id,
            name: selected_files[0].name.clone(),
            path: selected_files[0].path.clone(),
            attribute: selected_files[0].attribute.clone(),
            time: selected_files[0].time.clone(),
            noa: selected_files[0].noa,
            is_folder: selected_files[0].is_folder,
        }
    }

    pub fn queryFile_Bypath(&self, path: Option<String>) -> Vec<FileItem>{
        let selected_files: Vec<FileItem> =
        self.pool.prep_exec("SELECT * FROM file WHERE PATH = :path", 
                params!{"path" => path})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, path, attribute, time, noa, is_folder) = my::from_row(row);
                FileItem {
                    id: id,
                    name: name,
                    path: path,
                    attribute: attribute,
                    time: time,
                    noa: noa,
                    is_folder: is_folder,
                }
            }).collect()
        }).unwrap();
        selected_files
    }

    pub fn queryFragmentNumbers(&self, fileId: i32) -> i32{
        let selected_fragments: Vec<FragmentItem> =
        self.pool.prep_exec("SELECT * FROM fragment WHERE ID>=:id_1 AND ID<:id_2", 
                params!{"id_1" => fileId*100, "id_2" => (fileId+1)*100})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, path) = my::from_row(row);
                FragmentItem {
                    id: id,
                    path: path,
                }
            }).collect()
        }).unwrap();
        let mut i: i32 = 0;
        for _f in selected_fragments {
            i = i+1;
        }
        i
    }
    //no FragmentItem

    pub fn queryOnlineDevice(&self) -> Vec<DeviceItem> {
        let selected_devices: Vec<DeviceItem> =
        self.pool.prep_exec("SELECT * FROM DEVICE WHERE IS_ONLINE=true ORDER BY RS DESC", 
                ())
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, ip, port, is_online, rs) = my::from_row(row);
                DeviceItem {
                    id: id,
                    ip: ip,
                    port: port,
                    is_online: is_online,
                    rs: rs,
                }
            }).collect()
        }).unwrap();
        selected_devices
    }

    pub fn queryDevice(&self, id: i32) -> DeviceItem {
        let selected_devices: Vec<DeviceItem> =
        self.pool.prep_exec("SELECT * FROM DEVICE WHERE WHERE ID=:id", 
                params!{"id" => id})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, ip, port, is_online, rs) = my::from_row(row);
                DeviceItem {
                    id: id,
                    ip: ip,
                    port: port,
                    is_online: is_online,
                    rs: rs,
                }
            }).collect()
        }).unwrap();
        return DeviceItem {
            id: selected_devices[0].id,
            ip: selected_devices[0].ip.clone(),
            port: selected_devices[0].port.clone(),
            is_online: selected_devices[0].is_online,
            rs: selected_devices[0].rs,
        }
    }

    pub fn queryRequest_Byid(&self, id: i32) -> RequestItem {
        let selected_requests: Vec<RequestItem> =
        self.pool.prep_exec("SELECT * FROM REQUEST WHERE WHERE ID=:id", 
                params!{"id" => id})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, type_, fragmentId, deviceId) = my::from_row(row);
                RequestItem {
                    id: id,
                    type_: type_,
                    fragmentId: fragmentId,
                    deviceId: deviceId,
                }
            }).collect()
        }).unwrap();
        return RequestItem {
            id: selected_requests[0].id,
            type_: selected_requests[0].type_,
            fragmentId: selected_requests[0].fragmentId,
            deviceId: selected_requests[0].deviceId,
        }
    }

    pub fn queryFirstRequest_Byid(&self, id: i32) -> RequestItem {
        let selected_requests: Vec<RequestItem> =
        self.pool.prep_exec("SELECT * FROM REQUEST WHERE WHERE DEVICEID=:id LIMIT 1", 
                params!{"id" => id})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, type_, fragmentId, deviceId) = my::from_row(row);
                RequestItem {
                    id: id,
                    type_: type_,
                    fragmentId: fragmentId,
                    deviceId: deviceId,
                }
            }).collect()
        }).unwrap();
        return RequestItem {
            id: selected_requests[0].id,
            type_: selected_requests[0].type_,
            fragmentId: selected_requests[0].fragmentId,
            deviceId: selected_requests[0].deviceId,
        }
    }

    pub fn queryRequest_Bydeviceid(&self, deviceId: i32) -> RequestItem {
        let selected_requests: Vec<RequestItem> =
        self.pool.prep_exec("SELECT * FROM REQUEST WHERE WHERE DEVICEID=:id", 
                params!{"id" => deviceId})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, type_, fragmentId, deviceId) = my::from_row(row);
                RequestItem {
                    id: id,
                    type_: type_,
                    fragmentId: fragmentId,
                    deviceId: deviceId,
                }
            }).collect()
        }).unwrap();
        return RequestItem {
            id: selected_requests[0].id,
            type_: selected_requests[0].type_,
            fragmentId: selected_requests[0].fragmentId,
            deviceId: selected_requests[0].deviceId,
        }
    }

    pub fn queryRequestNumbers_Byid(&self, deviceId: i32) -> i32 {
        let selected_requests: Vec<RequestItem> =
        self.pool.prep_exec("SELECT * FROM REQUEST WHERE WHERE DEVICEID=:id", 
                params!{"id" => deviceId})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, type_, fragmentId, deviceId) = my::from_row(row);
                RequestItem {
                    id: id,
                    type_: type_,
                    fragmentId: fragmentId,
                    deviceId: deviceId,
                }
            }).collect()
        }).unwrap();
        let mut i: i32 = 0;
        for _r in selected_requests {
            i = i+1;
        }
        i
    }

    pub fn queryRequestNumbers_Byidtype(&self, fileId: i32, type_: i32) -> i32 {
        let selected_requests: Vec<RequestItem> =
        self.pool.prep_exec("SELECT * FROM REQUEST WHERE WHERE FRAGMENTID>=:fid
                AND FRAGMENTID<:fid2 AND TYPE_=:type_",
                params!{"fid" => fileId*100, "fid2" => (fileId+1)*100, "type_" => type_})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, type_, fragmentId, deviceId) = my::from_row(row);
                RequestItem {
                    id: id,
                    type_: type_,
                    fragmentId: fragmentId,
                    deviceId: deviceId,
                }
            }).collect()
        }).unwrap();
        let mut i: i32 = 0;
        for _r in selected_requests {
            i = i+1;
        }
        i
    }

    pub fn queryUserPasswd(&self, name: Option<String>) -> Option<String> {
        let selected_user: Vec<UserItem> =
        self.pool.prep_exec("SELECT * FROM USER WHERE NAME=:name",
                params!{"name" => &name})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, passwd) = my::from_row(row);
                UserItem {
                    id: id,
                    name: name,
                    passwd: passwd,
                }
            }).collect()
        }).unwrap();
        selected_user[0].passwd.clone()
    }

    pub fn queryUserID(&self, name: Option<String>) -> i32 {
        let selected_user: Vec<UserItem> =
        self.pool.prep_exec("SELECT * FROM USER WHERE NAME=:name",
                params!{"name" => &name})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, passwd) = my::from_row(row);
                UserItem {
                    id: id,
                    name: name,
                    passwd: passwd,
                }
            }).collect()
        }).unwrap();
        selected_user[0].id
    }
}

impl Query{
    pub fn addFile(&self,file:FileItem) -> i32{
        let suc:i32 = -1;
        if file.isFolder(){
            for mut stmt in self.pool.prepare(r"INSERT INTO DFS.FILE (NAME,PATH,ATTRIBUTE,TIME,NOA,ISFOLDER) 
                VALUES (:name,:path,:attribute,:time,:noa,true);").into_iter() {
                suc = stmt.execute(params!{
                    "name" => file.getName(),
                    "path" => file.getPath(),
                    "attribute" => file.getAttribute(),
                    "time" => file.getTime(),
                    "noa" => file.getNoa()
                }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功时，返回-1的情况
            }
        } else {
            for mut stmt in self.pool.prepare(r"INSERT INTO DFS.FILE (NAME,PATH,ATTRIBUTE,TIME,NOA,ISFOLDER) 
                VALUES (:name,:path,:attribute,:time,:noa,false);").into_iter() {
                suc = stmt.execute(params!{
                    "name" => file.getName(),
                    "path" => file.getPath(),
                    "attribute" => file.getAttribute(),
                    "time" => file.getTime(),
                    "noa" => file.getNoa()
                }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功时，返回-1的情况
            }
        }
        return suc;
    }

    pub fn deleteFile(&self,id:i32) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.FILE WHERE ID=:id").into_iter() {
            stmt.execute(params!{
                "id" => id
            }).unwrap();
        //此处未处理execute不成功时，返回-1的情况
        }
        suc = 1;
        return suc;
    }

    pub fn alterFile(&self,file:FileItem) -> i32{
        let suc:i32 = -1;
        if file.isFolder(){
            for mut stmt in self.pool.prepare(r"UPDATE DFS.FILE SET NAME=:name,PATH=:path,ATTRIBUTE=:attribute,
            TIME=:time,NOA=:noa,ISFOLDER=true WHERE id=:id;").into_iter() {
                stmt.execute(params!{
                    "name" => file.getName(),
                    "path" => file.getPath(),
                    "attribute" => file.getAttribute(),
                    "time" => file.getTime(),
                    "noa" => file.getNoa(),
                    "id" => file.getId() 
                }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功时，返回-1的情况
            }
        } else {
            for mut stmt in self.pool.prepare(r"UPDATE DFS.FILE SET NAME=:name,PATH=:path,ATTRIBUTE=:attribute,
            TIME=:time,NOA=:noa,ISFOLDER=false WHERE id=:id;").into_iter() {
                stmt.execute(params!{
                    "name" => file.getName(),
                    "path" => file.getPath(),
                    "attribute" => file.getAttribute(),
                    "time" => file.getTime(),
                    "noa" => file.getNoa(),
                    "id" => file.getId() 
                }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功时，返回-1的情况
            }
        }
        suc = 1;
        return suc;
    }

    pub fn alterDevice(&self,device:DeviceItem) -> i32{
        let suc:i32 = -1;
        if device.isOnline(){
            for mut stmt in self.pool.prepare(r"UPDATE DFS.DEVICE SET IP=:ip',PORT=:port,ISONLINE=true,
            RS=:rs WHERE id=:id;").into_iter() {
                let res = stmt.execute(params!{
                    "ip" => device.getIp(),
                    "port" => device.getPort(),
                    "rs" => device.getRs(),
                    "id" => device.getId()
                });
            }
        } else {
            for mut stmt in self.pool.prepare(r"UPDATE DFS.DEVICE SET IP=:ip,PORT=:port,ISONLINE=false,
            RS=:rsWHERE id=:id;").into_iter() {
                let res = stmt.execute(params!{
                    "ip" => device.getIp(),
                    "port" => device.getPort(),
                    "rs" => device.getRs(),
                    "id" => device.getId()
                });   
                suc = match res{
                    Ok(_) => 1,
                    Err(_) => -1
                };
            }
        }       
        return suc;
    }

    pub fn addFragment(&self,id:i32,path:String) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"INSERT INTO DFS.FRAGMENT VALUES (:id,:path)").into_iter() {
            let res = stmt.execute(params!{
                "id" => id,
                "path" => path
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    } 

    pub fn deleteFragment(&self,id:i32) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.FRAGMENT WHERE ID=:id").into_iter() {
            let res = stmt.execute(params!{
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }

    pub fn alterFragment(&self,id:i32,path:String) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"UPDATE DFS.FRAGMENT SET PATH=:path WHERE id=:id;").into_iter() {
            let res = stmt.execute(params!{
                "path" => path,
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }

    pub fn addRequest(&self,request:RequestItem) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"INSERT INTO DFS.REQUEST (TYPE,FRAGMENTID,DEVICEID)
        VALUES (:type,:fragmentid,:deviceid)").into_iter() {
            suc = stmt.execute(params!{
                "type" => request.getType(),
                "fragmentid" => request.getFragementId(),
                "deviceid" => request.getDeviceId()
            }).unwrap().last_insert_id().try_into().unwrap();
        //此处未处理execute不成功时，返回-1的情况
        }
        suc = 1;
        return suc;
    }

    pub fn deleteRequest(&self,id:i32) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.REQUEST WHERE ID=:id").into_iter() {
            let res = stmt.execute(params!{
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }
    pub fn addUser(&self,name:String,passwd:String)-> i32{
        //suc为INSERT的用户在mysql中的id
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"INSERT INTO DFS.USER (NAME,PASSWD) VALUES (:name, :passwd)").into_iter() {
            suc = stmt.execute(params!{
                "name" => name,
                "passwd" => passwd
            }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功的情况
        }
        suc = 1;
        return suc;
    }

    pub fn alterUser(&self,id:i32,name:String,passwd:String) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"UPDATE INTO DFS.USER SET NAEME=:name,PASSWD=passwd WHERE id=:id").into_iter() {
            let res = stmt.execute(params!{
                "name" => name,
                "passwd" => passwd,
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }

    pub fn deleteUser(&self,id:i32) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.USER WHERE ID=:id").into_iter() {
            let res = stmt.execute(params!{
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }
}