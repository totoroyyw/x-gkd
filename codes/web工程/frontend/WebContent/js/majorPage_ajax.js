$(document).ready(function(){
	var curr_path_array = new Array();
	var hour = 0;
	var minute = 0;
	var second = 0;
	var millisecond = 0;
	var int;
	curr_path_array[0] = "/";
	curr_path_html = "<li>ROOT</li>";
	
	//面包屑式访问路径显示  初始化
	$("#curr_path").html(curr_path_html);
	
	//文件下载
	$("#button_download").click(
	function()
		{
			var path1;
			var name1;

			//重置时钟
			window.clearInterval(int);
			millisecond = second = minute = hour = 0;
			//document.getElementById("timetext").value = '00时00分00秒000毫秒';
			//

			var item=$("#file_list_body").children();
			item = item.next();
			while(item.length!=0)
				{
					name1 = "";
					path1 = "";
					//如果ｉｔｅｍ不为空，则进行处理
					var children=item.children();
					if( (children[1].children[1].className=="glyphicon glyphicon-file") && (children[1].children[0].children[0].checked) )
						{
							//文件路径
							path1 = path1 + "/";
/*********/					if(curr_path_array.length>1)
								path1="";
							for(var i=1;i<curr_path_array.length;i++)
								path1 = path1 + curr_path_array[i] + "/" ;
							//文件名
							name1 = name1 + $.trim(children[1].innerText);
							//alert(path + "  " + name);
							
							
							/*
							 * 
							 * 此处应当利用ａｊａｘ　远程调用　downloadRegister(String path, String name)；
							 * 
							 * */
							//利用ａｊａｘ　远程调用　downloadRegister(String path, String name)；

							//从向后端发送请求开始计时
							function timer() {
								millisecond = millisecond + 50;
								if(millisecond>=1000)
								{
									millisecond=0;
									second=second+1;
								}
								if(second>=60)
								{
									second=0;
									minute=minute+1;
								}

								if(minute>=60)
								{
									minute=0;
									hour=hour+1;
								}
								//document.getElementById('timetext').value=hour+'时'+minute+'分'+second+'秒'+millisecond+'毫秒';

							}
							int = setInterval(timer, 50);



							var result;
							$.ajax({
								url:"http://127.0.0.1:8000/DownloadReg",
								type:"POST",
								data:JSON.stringify({
									path:path1,
									name:name1,
								}),
								dataType:"json",
								contentType:"application/json; charset=utf-8",
								async: false,								//此处采用同步查询进度
								success:function(databack){
									//var obj = $.parseJSON(databack);
									result = databack.result;

									alert(result);
								}
							});

							//$("#statusFeedback").text("下载已运行！");

							//错误处理
							if(result=="NotEnoughFragments")
							{
								$("#statusFeedback").text("在线碎片数目不足！");
								return;
							}
							else if(result == "Error")
							{
								$("#statusFeedback").text("服务器响应该请求内部出错！");
								return;
							}
							
							//添加进度条
							var ratio１ = 0;
							var progress_bar='<div class="progress progress-striped active"><div class="progress-bar progress-bar-success" role=\"progressbar" style="width: '
								+ratio１+'%;">'
								+path1+name1+'</div></div>';
							$("#download_progress_area").append(progress_bar);
							
						}
					//
					item = item.next();
				}
		}
	);
	/*
		<tr id="file_list_first">
		<td> </td>
 		<td> <label><input type="checkbox">&emsp;&emsp;</label><span class="glyphicon glyphicon-folder-open"></span>&emsp;../</td>
 		<td></td>
 		<td></td>
		</tr>

*/
	
	
	
	
	
	//点击文件目录进入其子目录　　刷新文件目录列表
	$("#file_list_body").on("click","tr.file_list_go",
			function()
			{
				//如果是文件而不是文件夹，点击不刷新目录，提示信息
				if(this.children[1].children[1].className=="glyphicon glyphicon-file")
				{
					$("#statusFeedback").text("您所点击的是文件而不是文件夹，无法进入该目录！");
					return;
				}
				//更新路径显示
				curr_path_array = curr_path_array.concat( $.trim(this.children[1].innerText) );			//此处用$.trim去除空格
				curr_path_html = "<li>ROOT</li>";
				for(var i=1;i<curr_path_array.length;i++)
				curr_path_html = curr_path_html + "<li>" + curr_path_array[i] + "</li>";
				$("#curr_path").html(curr_path_html);		
				//ajax
				var QueryPath1="/";
/*********/		if(curr_path_array.length>1)
					QueryPath1="";
				for(var i=1;i<curr_path_array.length;i++)
				{
					QueryPath1 = QueryPath1 + curr_path_array[i] + "/" ;
				}
				
				//alert(queryPath);
				$.ajax({
						url:"http://127.0.0.1:8000/GetFileList",
						type:"POST",
						data:JSON.stringify({
							QueryPath:QueryPath1,
						}),
						dataType:"json",
						contentType:"application/json; charset=utf-8",
						success:function(databack){
							//var obj = $.parseJSON(databack);
							var new_file_list = databack.result;//html字符串
							//alert(new_file_list);
							$("#file_list_body").html(new_file_list);
						}
				});
				$("#statusFeedback").text("成功进入该目录！");
			}
	);
	
	//点击的是返回上一层的文件项
	$("#file_list_body").on("click","tr.file_list_back",
			function()
			{
				//如果是顶层目录，点击上级目录无操作，提示信息
				if(curr_path_array.length==1)
				{
					$("#statusFeedback").text("已经是根目录了，无法返回上一层！");
					return; 
				}
				//更新路径显示
				curr_path_array.pop();
				curr_path_html = "<li>ROOT</li>";
				for(var i=1;i<curr_path_array.length;i++)
				curr_path_html = curr_path_html + "<li>" + curr_path_array[i] + "</li>";
				$("#curr_path").html(curr_path_html);	
				
				//ajax
				var QueryPath1="/";
/*********/		if(curr_path_array.length>1)
					QueryPath1="";
				for(var i=1;i<curr_path_array.length;i++)
				{
					QueryPath1 = QueryPath1 + curr_path_array[i] + "/" ;
				}
				
				//alert(queryPath);
				$.ajax({
						url:"http://127.0.0.1:8000/GetFileList",
						type:"POST",
						data:JSON.stringify({
							QueryPath:QueryPath1,
						}),
						dataType:"json",
						contentType:"application/json; charset=utf-8",
						success:function(databack){
							//var obj = $.parseJSON(databack);
							var new_file_list = databack.result;//html字符串
							//alert(new_file_list);
							$("#file_list_body").html(new_file_list);
						}
				});
				$("#statusFeedback").text("成功返回上层目录！");
			}
	);

	
	//定时刷新预下载进度
	function refresh_progress(){
		var progressArray = $("#download_progress_area").children();
		var str="";
		var ratio=100;
		for(var i=0;i<progressArray.length;i++)
		if(progressArray[i].className=="progress progress-striped active")
		{
			//alert("here length="+progressArray.length + "i="+i);
			var path1="";
			var name1="";
			var strArray;
			strArray = progressArray[i].innerText.split('/');
			for(var j=0;j<strArray.length-1;j++)
				path1 = path1 + strArray[j] + "/";
			name1 = strArray[strArray.length-1];
			//str = str + path + name + "    ";
			//alert(name+" "+path)
			/*
			 * 
			 * 此处应远程调用　public static int progressCheck(String path, String name)　　返回进度
			 * 
			 * */
			$.ajax({
					url:"http://127.0.0.1:8000/progressCheck",
					type:"POST",
					data:JSON.stringify({
						path:path1,
						name:name1,
					}),
					dataType:"json",
					contentType:"application/json; charset=utf-8",
					async: false,								//此处采用同步查询进度
					success:function(databack){
						//var obj = $.parseJSON(databack);
						var result = databack.result;
						if(result == "Error")
						{
							ratio = 0;
							$("#statusFeedback").text("查询进度出错！");
						}
						else
							ratio = parseInt(result);

					}
			});

			//////////////////////////////////////////////////////////////////
			//进度条的ｈｔｍｌ代码
			var progress_bar='<div class="progress progress-striped active"><div class="progress-bar progress-bar-success" role=\"progressbar" style="width: '
				+ratio+'%;">'
				+path1+name1+'</div></div>';
			//如果预下载完成
			if(ratio==100)
			{
				/*
				 * 
				 * 
				 * 此处应当调用远程函数　　public static int decodeFile(String path, String name)
				 * 
				 * */
				$.ajax({
						url:"http://127.0.0.1:8000/decodeFile",
						type:"POST",
						data:JSON.stringify({
							path:path1,
							name:name1,
						}),
						dataType:"json",
						contentType:"application/json; charset=utf-8",
						async: false,								//此处采用同步查询进度
						success:function(databack){
							//var obj = $.parseJSON(databack);
							var result = databack.result;

							//停止计时
							window.clearInterval(int);

							if(result == "Error")
								$("#statusFeedback").text("解码拼接出错！");

							else{
								var time_temp = hour + "时" + minute + "分" + second + "秒" + millisecond + "毫秒";
								$("#statusFeedback").text("解码拼接文件成功,用时").append(time_temp);
							}



						}
				});
				
				
				var temp = '<a href="/DFS/CloudDriveServer/tmpFile/' + name1 + '" download="' + name1 + '">' + progress_bar + '</a>';
				//alert(temp);
				progressArray[i].outerHTML = temp;	
				
			}
			else
			{
				//修改进度条进度
				//alert(progress_bar);
				progressArray[i].outerHTML = progress_bar;				
			}
			///////////////////////////////////////////////////////
		}
		
	}
	//设置进度刷新间隔
	window.setInterval(function(){refresh_progress();},3000);

	
	
	
	//自动删除下载过的文件链接和进度条
	$("#download_progress_area").on("click","a",
			function()
			{
				this.outerHTML = "";
			}

	);
	
	
	
//总的结束符	
});

/*
   			<tr id="file_list_first">
      			<td> </td>
         		<td> <label><input type="checkbox">&emsp;&emsp;</label><span class="glyphicon glyphicon-folder-open"></span>&emsp;../</td>
         		<td></td>
         		<td></td>
      		</tr>
 
 */